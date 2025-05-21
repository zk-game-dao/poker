use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    http::HeaderValue,
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};
use candid::{Decode, Encode, Principal};
use debug::create_debug_router;
use discord::{send_discord_message, DiscordClient};
use dotenv::dotenv;
use errors::{table_error::TableError, table_index_error::TableIndexError};
use feedback::{Feedback, FeedbackError};
use ic_agent::{
    agent::{Agent, AgentError},
    identity::AnonymousIdentity,
};
use notifications::{
    delete_push_subscription, get_push_subscription, register_push_subscription,
    send_push_notifications, PushClient,
};
use reqwest::{Method, StatusCode};
use serde_json::{json, Value};
use std::{collections::HashMap, sync::Arc, time::Duration};
use table::poker::game::{
    table_functions::{table::TableConfig, types::Notification},
    types::PublicTable,
};
use telegram::{send_telegram_message, TelegramClient};
use tokio::sync::{broadcast, Mutex};
use tower_http::cors::{Any, CorsLayer};
use twitter::{send_tweet, TwitterClient};

pub mod debug;
pub mod discord;
pub mod feedback;
pub mod notifications;
pub mod telegram;
pub mod twitter;

// State tracking structures
#[derive(Debug, Clone)]
pub struct TableState {
    player_count: usize,
    last_notifications: Vec<Notification>,
}

pub type TableStates = Arc<Mutex<HashMap<Principal, TableState>>>;
type Broadcaster = broadcast::Sender<HashMap<Principal, Vec<Notification>>>;

pub struct AppState {
    pub table_states: TableStates,
    pub broadcaster: Arc<Broadcaster>,
    pub telegram_client: TelegramClient,
    pub telegram_channel_id: String,
    pub twitter_client: TwitterClient,
    pub push_client: PushClient,
    pub discord_client: DiscordClient,
    pub table_index_id: Principal,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let table_states: TableStates = Arc::new(Mutex::new(HashMap::new()));

    // Create broadcast channel for notifications
    let (tx, _rx) = broadcast::channel::<HashMap<Principal, Vec<Notification>>>(16);
    let broadcaster = Arc::new(tx);

    let bot_token = std::env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let twitter_consumer_key =
        std::env::var("TWITTER_CONSUMER_KEY").expect("TWITTER_CONSUMER_KEY not set");
    let twitter_consumer_secret =
        std::env::var("TWITTER_CONSUMER_SECRET").expect("TWITTER_CONSUMER_SECRET not set");
    let twitter_access_token =
        std::env::var("TWITTER_ACCESS_TOKEN").expect("TWITTER_ACCESS_TOKEN not set");
    let twitter_access_token_secret =
        std::env::var("TWITTER_ACCESS_TOKEN_SECRET").expect("TWITTER_ACCESS_TOKEN_SECRET not set");
    let table_index_id = Principal::from_text(
        std::env::var("CANISTER_ID_TABLE_INDEX").expect("CANISTER_ID_TABLE_INDEX not set"),
    )
    .expect("Invalid table index principal");
    let vapid_file = std::env::var("VAPID_FILE").expect("VAPID_FILE not set");
    let cors = std::env::var("WEBSERVER_CORS").unwrap_or_else(|_| "*".to_string());

    let telegram_client = TelegramClient::new(bot_token.clone());

    let twitter_client = TwitterClient::new(
        twitter_consumer_key,
        twitter_consumer_secret,
        twitter_access_token,
        twitter_access_token_secret,
    );

    let push_client = PushClient::new(vapid_file).expect("Failed to create push client");

    let discord_webhook_url =
        std::env::var("DISCORD_WEBHOOK_URL").expect("DISCORD_WEBHOOK_URL not set");
    let discord_client = DiscordClient::new(discord_webhook_url);

    // Create app state
    let app_state = Arc::new(AppState {
        table_states: table_states.clone(),
        broadcaster: broadcaster.clone(),
        telegram_client,
        telegram_channel_id: std::env::var("TELEGRAM_CHANNEL_ID")
            .expect("TELEGRAM_CHANNEL_ID not set"),
        twitter_client,
        push_client,
        discord_client,
        table_index_id,
    });

    // Spawn the table polling task
    {
        let app_state = app_state.clone();
        tokio::spawn(async move {
            if let Err(e) = poll_tables_task(app_state).await {
                eprintln!("Table polling task encountered an error: {:?}", e);
            }
        });
    }

    // Build a CORS layer depending on the value of cors
    let cors_layer = if cors == "*" {
        // Allow all origins
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any)
    } else {
        // Allow only the specified origin
        // (Assumes the origin is a valid URI; handle errors appropriately in production)
        let origin = cors
            .parse::<HeaderValue>()
            .expect("Invalid WEBSERVER_CORS value");
        CorsLayer::new()
            .allow_origin(origin)
            .allow_methods([Method::GET, Method::POST])
            .allow_headers(Any)
    };

    // Build the Axum application with routes
    let app = Router::new()
        .route("/ws", get(ws_handler))
        .route("/get-public-key", get(get_public_key))
        .route(
            "/notification/{principal}",
            post(register_push_subscription),
        )
        .route("/notification/{principal}", get(get_push_subscription))
        .route(
            "/notification/{principal}",
            delete(delete_push_subscription),
        )
        .nest("/debug", create_debug_router())
        .route("/feedback", post(handle_feedback))
        .with_state(app_state)
        .layer(cors_layer);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl axum::response::IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state.broadcaster.clone()))
}

pub async fn get_public_key(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Value>, (StatusCode, String)> {
    match state.push_client.get_public_key() {
        Ok(key) => Ok(Json(json!({ "publicKey": key }))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e))),
    }
    // let key: String = state.push_client.get_public_key().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    // Ok(Json(json!({ "publicKey": key })))
}

async fn handle_feedback(
    Json(feedback): Json<Feedback>,
) -> Result<impl IntoResponse, FeedbackError> {
    feedback.post_to_github().await?;
    Ok((
        StatusCode::OK,
        "Feedback submitted successfully".to_string(),
    ))
}

async fn handle_socket(mut socket: WebSocket, broadcaster: Arc<Broadcaster>) {
    let mut rx = broadcaster.subscribe();

    loop {
        tokio::select! {
            Ok(notifications) = rx.recv() => {
                if let Ok(json) = serde_json::to_string(&notifications) {
                    if socket.send(Message::Text(json.into())).await.is_err() {
                        break;
                    }
                }
            }
            result = socket.recv() => {
                match result {
                    Some(Ok(msg)) => {
                        if let Message::Close(_) = msg {
                            break;
                        }
                    },
                    _ => break,
                }
            }
        }
    }
}

async fn poll_tables_task(app_state: Arc<AppState>) -> Result<(), Box<dyn std::error::Error>> {
    let agent = get_agent().await?;
    let mut interval = tokio::time::interval(Duration::from_secs(5));

    loop {
        interval.tick().await;

        match get_public_table_principals(app_state.table_index_id, &agent).await {
            Ok(principals) => {
                for (principal, _) in principals {
                    if let Ok(player_count) = get_table_player_count(&agent, &principal).await {
                        // Get table information
                        if let Ok(notifications) = poll_notifications(&agent, &principal).await {
                            process_table_update(
                                &app_state,
                                principal,
                                player_count,
                                notifications,
                            )
                            .await;
                        }
                    }
                }
            }
            Err(err) => {
                eprintln!("Error getting table principals: {:?}", err);
            }
        }
    }
}

async fn process_table_update(
    app_state: &AppState,
    principal: Principal,
    current_player_count: usize,
    current_notifications: Vec<Notification>,
) {
    let mut states = app_state.table_states.lock().await;
    let state = states.entry(principal).or_insert(TableState {
        player_count: current_player_count,
        last_notifications: Vec::new(),
    });

    // Check for player count changes
    if state.player_count != current_player_count && current_player_count > 0 {
        send_player_count_update(
            app_state,
            principal,
            state.player_count,
            current_player_count,
        )
        .await;
    }
    state.player_count = current_player_count;

    // Check for new notifications
    let new_notifications: Vec<_> = current_notifications
        .iter()
        .filter(|n| !state.last_notifications.contains(n))
        .cloned()
        .collect();

    if !new_notifications.is_empty() {
        send_notifications(app_state, principal, &new_notifications).await;
        state.last_notifications = current_notifications;
    }
}

async fn send_player_count_update(
    app_state: &AppState,
    table: Principal,
    old_count: usize,
    new_count: usize,
) {
    // Send Telegram message
    let telegram_msg = format!(
        "Table {} player count changed from {} to {}",
        table, old_count, new_count
    );
    println!("Sending Telegram message: {}", telegram_msg);
    send_telegram_message(
        &app_state.telegram_client,
        &app_state.telegram_channel_id,
        &table.to_text(),
        old_count,
        new_count,
    )
    .await;

    // Send Tweet
    let tweet_msg = format!(
        "Table {} now has {} players! Join the action! 🎮",
        table, new_count
    );
    println!("Sending Tweet: {}", tweet_msg);
    send_tweet(
        &app_state.twitter_client,
        &table.to_text(),
        old_count,
        new_count,
    )
    .await;

    // Send Discord message
    println!("Sending Discord message");

    send_discord_message(
        &app_state.discord_client,
        &table.to_text(),
        old_count,
        new_count,
    )
    .await;
}

async fn send_notifications(
    app_state: &AppState,
    table: Principal,
    notifications: &[Notification],
) {
    // Group notifications by user
    let mut user_notifications: HashMap<Principal, Vec<Notification>> = HashMap::new();
    for notification in notifications {
        user_notifications
            .entry(notification.user_principal)
            .or_default()
            .push(notification.clone());
    }

    // Send push notifications
    for (user, notifications) in user_notifications {
        println!("Sending push notification to {}: {:?}", user, notifications);
        if let Err(e) =
            send_push_notifications(&app_state.push_client, table, user, &notifications).await
        {
            eprintln!("Error sending push notification: {:?}", e);
        }
    }

    // Broadcast to websocket clients
    let mut broadcast_map = HashMap::new();
    broadcast_map.insert(table, notifications.to_vec());
    if let Err(e) = app_state.broadcaster.send(broadcast_map) {
        eprintln!("Error broadcasting notifications: {:?}", e);
    }
}

/// Get an IC agent for making canister calls
async fn get_agent() -> Result<Agent, AgentError> {
    dotenv().ok();
    let url = &std::env::var("IC_URL").unwrap_or_else(|_| "https://ic0.app".to_string());
    println!("Using IC URL: {}", url);
    let identity = AnonymousIdentity {};
    let agent = Agent::builder()
        .with_url(url)
        .with_identity(identity)
        .build()?;

    if url == "http://127.0.0.1:4943" {
        agent.fetch_root_key().await?;
    }

    Ok(agent)
}

/// Get all public table principals from the index
async fn get_public_table_principals(
    table_index_id: Principal,
    agent: &Agent,
) -> Result<Vec<(Principal, TableConfig)>, AgentError> {
    let result = agent
        .query(&table_index_id, "get_all_public_tables")
        .with_arg(Encode!()?)
        .call()
        .await?;

    let principals = Decode!(
        &result,
        Result<Vec<(Principal, TableConfig)>, TableIndexError>
    )
    .map_err(|e| AgentError::CandidError(Box::new(e)))?;
    Ok(principals.unwrap_or_default())
}

/// Query a table's player count
async fn get_table_player_count(agent: &Agent, table_id: &Principal) -> Result<usize, AgentError> {
    let result = agent
        .query(table_id, "get_table")
        .with_arg(Encode!()?)
        .call()
        .await?;

    let table = Decode!(&result, Result<PublicTable, TableError>)
        .map_err(|e| AgentError::CandidError(Box::new(e)))?;
    let table = table.map_err(|e| AgentError::CandidError(Box::new(e)))?;

    Ok(table.users.len())
}

/// Query a table for notifications
async fn poll_notifications(
    agent: &Agent,
    table_id: &Principal,
) -> Result<Vec<Notification>, AgentError> {
    let result = agent
        .query(table_id, "get_notifications")
        .with_arg(Encode!()?)
        .call()
        .await?;

    let notifications = Decode!(&result, Result<Vec<Notification>, TableError>)
        .map_err(|e| AgentError::CandidError(Box::new(e)))?;
    Ok(notifications.unwrap_or_default())
}
