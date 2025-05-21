use axum::{
    extract::{Path, State},
    Json, Router,
};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use candid::Principal;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{collections::HashMap, fs::File, sync::Arc};
use table::poker::game::table_functions::types::{Notification, NotificationMessage};
use tokio::sync::Mutex;
use web_push::{
    ContentEncoding, IsahcWebPushClient, SubscriptionInfo, VapidSignatureBuilder, WebPushClient,
    WebPushMessageBuilder,
};

use crate::AppState;

// Store for user subscriptions
type SubscriptionStore = Arc<Mutex<HashMap<Principal, PushSubscription>>>;

#[derive(Debug)]
pub enum PushError {
    VapidError(String),
    SubscriptionError(String),
    SendError(String),
    NoSubscription(Principal),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PushSubscription {
    pub endpoint: String,
    pub keys: PushKeys,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PushKeys {
    pub auth: String,
    pub p256dh: String,
}

impl PushSubscription {
    fn into_subscription_info(self) -> SubscriptionInfo {
        SubscriptionInfo::new(self.endpoint, self.keys.p256dh, self.keys.auth)
    }
}

pub struct PushClient {
    vapid_file: String,
    client: IsahcWebPushClient,
    subscriptions: SubscriptionStore,
}

impl std::fmt::Debug for PushClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PushClient")
            .field("vapid_file", &self.vapid_file)
            .field("subscriptions", &self.subscriptions)
            .finish()
    }
}

impl PushClient {
    pub fn new(vapid_file: String) -> Result<Self, PushError> {
        let client = IsahcWebPushClient::new().map_err(|e| PushError::VapidError(e.to_string()))?;

        Ok(Self {
            vapid_file,
            client,
            subscriptions: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    pub async fn register_subscription(
        &self,
        principal: Principal,
        subscription: PushSubscription,
    ) -> Result<(), PushError> {
        let mut subs = self.subscriptions.lock().await;
        subs.insert(principal, subscription);
        Ok(())
    }

    pub async fn remove_subscription(&self, principal: Principal) -> Result<(), PushError> {
        let mut subs = self.subscriptions.lock().await;
        subs.remove(&principal);
        Ok(())
    }

    pub async fn get_subscription(&self, principal: &Principal) -> Option<PushSubscription> {
        let subs = self.subscriptions.lock().await;
        subs.get(principal).cloned()
    }

    pub async fn send_notification(
        &self,
        table_principal: Principal,
        user_principal: Principal,
        notifications: &[Notification],
    ) -> Result<(), PushError> {
        let subscription = self
            .get_subscription(&user_principal)
            .await
            .ok_or(PushError::NoSubscription(user_principal))?;

        let subscription_info = subscription.into_subscription_info();

        // Read VAPID private key
        let file = File::open(&self.vapid_file)
            .map_err(|e| PushError::VapidError(format!("Failed to read VAPID file: {}", e)))?;

        // Build VAPID signature
        let sig_builder = VapidSignatureBuilder::from_pem(file, &subscription_info)
            .map_err(|e| PushError::VapidError(format!("Failed to build VAPID signature: {}", e)))?
            .build()
            .map_err(|e| {
                PushError::VapidError(format!("Failed to build VAPID signature: {}", e))
            })?;

        // Create message payload
        let formatted_notifications: Vec<_> = notifications
            .iter()
            .map(|n| format_notification_content(table_principal, n))
            .collect();

        let content = serde_json::to_string(&formatted_notifications).map_err(|e| {
            PushError::SendError(format!("Failed to serialize notifications: {}", e))
        })?;

        // Build the push message
        let mut builder = WebPushMessageBuilder::new(&subscription_info);
        builder.set_payload(ContentEncoding::Aes128Gcm, content.as_bytes());
        builder.set_vapid_signature(sig_builder);

        let message = builder
            .build()
            .map_err(|e| PushError::SendError(format!("Failed to build message: {}", e)))?;

        // Send the notification
        self.client.send(message).await.map_err(|e| {
            PushError::SendError(format!("Failed to send push notification: {}", e))
        })?;

        Ok(())
    }

    pub fn subscription_routes() -> Router {
        Router::new()
    }

    pub fn get_public_key(&self) -> Result<String, PushError> {
        // Read VAPID private key
        let file = File::open(&self.vapid_file)
            .map_err(|e| PushError::VapidError(format!("Failed to read VAPID file: {}", e)))?;

        let vap = VapidSignatureBuilder::from_pem_no_sub(file)
            .map_err(|e| PushError::VapidError(format!("Failed to build VAPID key: {}", e)))?;

        Ok(STANDARD.encode(vap.get_public_key()))
    }
}

// Function to format poker notification content
fn format_notification_content(
    table_principal: Principal,
    notification: &Notification,
) -> serde_json::Value {
    match notification.message {
        NotificationMessage::UserTurnStarted => json!({
            "title": "Your turn",
            "body": "It's your turn to play",
            "table": table_principal.to_text(),
        }),
    }
}

pub async fn send_push_notifications(
    push_client: &PushClient,
    table_principal: Principal,
    user_principal: Principal,
    notifications: &[Notification],
) -> Result<(), PushError> {
    println!(
        "Sending push notifications to {}: {:?}",
        user_principal, notifications
    );
    push_client
        .send_notification(table_principal, user_principal, notifications)
        .await
}

pub async fn register_push_subscription(
    State(state): State<Arc<AppState>>,
    Path(principal_str): Path<String>,
    Json(subscription): Json<PushSubscription>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let principal = Principal::from_text(&principal_str)
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    match state
        .push_client
        .register_subscription(principal, subscription)
        .await
    {
        Ok(_) => Ok(Json(json!({ "status": "success" }))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e))),
    }
}

pub async fn get_push_subscription(
    State(state): State<Arc<AppState>>,
    Path(principal_str): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let principal = Principal::from_text(&principal_str)
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    match state.push_client.get_subscription(&principal).await {
        Some(sub) => Ok(Json(json!(sub))),
        _ => Err((StatusCode::NOT_FOUND, "No subscription found".to_string())),
    }
}

pub async fn delete_push_subscription(
    State(state): State<Arc<AppState>>,
    Path(principal_str): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let principal = Principal::from_text(&principal_str)
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    match state.push_client.remove_subscription(principal).await {
        Ok(_) => Ok(Json(json!({ "status": "success" }))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e))),
    }
}
