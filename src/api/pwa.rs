use axum::{Json, extract::State};

use crate::Cfg;

pub async fn enabled(State(state): State<Cfg>) -> Json<bool> {
    Json(state.server.pwa_enabled)
}
