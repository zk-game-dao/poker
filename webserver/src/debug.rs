use std::collections::HashMap;
use std::sync::Arc;

use axum::body::Body;
use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    routing::{get, Router},
    Json,
};

use crate::AppState;

async fn show_debug_vars() -> Result<Json<HashMap<String, String>>, (StatusCode, String)> {
    let env_vars: HashMap<String, String> = std::env::vars().collect();
    Ok(Json(env_vars))
}

async fn show_debug_appstate(
    State(state): State<Arc<AppState>>,
) -> Result<Json<HashMap<String, String>>, (StatusCode, String)> {
    let mut app_state = HashMap::new();

    app_state.insert(
        "table_states".to_string(),
        format!("{:?}", state.table_states),
    );
    app_state.insert(
        "broadcaster".to_string(),
        format!("{:?}", state.broadcaster),
    );
    app_state.insert(
        "telegram_client".to_string(),
        format!("{:?}", state.telegram_client),
    );
    app_state.insert(
        "telegram_channel_id".to_string(),
        format!("{:?}", state.telegram_channel_id),
    );
    app_state.insert(
        "twitter_client".to_string(),
        format!("{:?}", state.twitter_client),
    );
    app_state.insert(
        "push_client".to_string(),
        format!("{:?}", state.push_client),
    );
    app_state.insert(
        "discord_client".to_string(),
        format!("{:?}", state.discord_client),
    );
    app_state.insert(
        "table_index_id".to_string(),
        format!("{:?}", state.table_index_id),
    );

    Ok(Json(app_state))
}

async fn admin_token_check(req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    const VALID_TOKEN: &str = "PiNACzWTl3TYVRX22B9lNB1l4WDYpdHTtnlrsIWvuwGrSxrRmn";

    if let Some(token) = req.headers().get("X-API-TOKEN") {
        if let Ok(token_str) = token.to_str() {
            if token_str == VALID_TOKEN {
                return Ok(next.run(req).await);
            }
        }
    }
    Err(StatusCode::UNAUTHORIZED)
}

pub fn create_debug_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/vars", get(show_debug_vars))
        .route("/state", get(show_debug_appstate))
        .layer(axum::middleware::from_fn(admin_token_check))
}
