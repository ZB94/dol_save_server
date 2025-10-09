use axum::{
    extract::{Request, State},
    http::{Uri, header},
    middleware::Next,
    response::Response,
};

use crate::Cfg;

#[derive(Debug, Clone)]
pub struct GameName(pub Option<String>);

pub async fn layer_game_name(State(cfg): State<Cfg>, mut request: Request, next: Next) -> Response {
    let mut name = None;

    if let Some(referer) = request
        .headers()
        .get(header::REFERER)
        .and_then(|h| h.to_str().ok())
        .and_then(|uri| uri.parse::<Uri>().ok())
    {
        set_name(&cfg, &referer, &mut name);
    }
    trace!("游戏名称(referer): {name:?}");

    if name.is_none() {
        set_name(&cfg, request.uri(), &mut name);
    }
    debug!("游戏名称: {name:?}");

    request.extensions_mut().insert(GameName(name));

    next.run(request).await
}

fn set_name(cfg: &Cfg, uri: &Uri, name: &mut Option<String>) {
    let path = uri.path().trim_matches('/');
    if !path.is_empty() && cfg.game.iter().any(|g| g.name == path) {
        *name = Some(path.to_string());
    }
}
