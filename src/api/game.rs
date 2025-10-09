use axum::{Json, extract::State};

use crate::Cfg;

pub async fn list(State(cfg): State<Cfg>) -> Json<Vec<String>> {
    Json(cfg.game.iter().map(|g| g.name.clone()).collect())
}
