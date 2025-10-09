pub mod code;
pub mod list;
pub mod remove;
#[allow(clippy::module_inception)]
pub mod save;

use axum::{
    Extension, Json,
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
pub use code::code;
pub use list::list;
pub use remove::remove;
pub use save::save;

use crate::{Cfg, config::game::Game};

#[derive(Debug, Clone)]
pub struct GameName(pub String);

pub async fn layer_game_name(
    Extension(game_name): Extension<crate::web::game_name::GameName>,
    State(cfg): State<Cfg>,
    mut request: Request,
    next: Next,
) -> Response {
    if let Some(name) = game_name.0
        && cfg.game.iter().any(|g| g.name == name)
    {
        request.extensions_mut().insert(GameName(name));
        next.run(request).await
    } else {
        (StatusCode::NOT_FOUND, Json("未找到对应游戏")).into_response()
    }
}

fn game<'g>(name: &str, cfg: &'g Cfg) -> &'g Game {
    cfg.game
        .iter()
        .find(|g| g.name == name)
        .unwrap_or(&cfg.game[0])
}
