use axum::{Extension, Json, extract::State};

use crate::{Cfg, web::game_name::GameName};

pub async fn list(
    Extension(GameName(current)): Extension<GameName>,
    State(cfg): State<Cfg>,
) -> Json<serde_json::Value> {
    let game = cfg.game.iter().map(|g| g.name.clone()).collect::<Vec<_>>();
    Json(serde_json::json!({
        "list": game,
        "current": current
    }))
}
