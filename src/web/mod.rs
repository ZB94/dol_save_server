use std::{convert::Infallible, io::ErrorKind, path::Path};

use axum::{
    Extension,
    extract::{Request, State},
    http::{StatusCode, header},
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use include_dir::{Dir, include_dir};

use crate::{Cfg, web::game_name::GameName};

pub mod game_name;

const WEB: Dir = include_dir!("web");

#[instrument(skip_all)]
pub async fn web_service(
    Extension(GameName(game)): Extension<GameName>,
    State(cfg): State<Cfg>,
    request: Request,
) -> Result<Response, Infallible> {
    let path = request.uri().path();
    debug!(path, "web request path");

    let path = path.trim_start_matches('/');

    if let Some(file) = WEB.get_file(path) {
        let mut resp = file.contents().into_response();
        set_content_type(&mut resp, path);

        Ok(resp)
    } else {
        if path.is_empty() {
            return Ok(Redirect::to("/game.html").into_response());
        }

        let game = game
            .and_then(|game| cfg.game.iter().find(|g| g.name == game))
            .unwrap_or(&cfg.game[0]);

        let path = if path == game.name {
            Path::new(&game.index).to_path_buf()
        } else {
            game.root.join(path)
        };

        debug!("path: {path:?}");

        match tokio::fs::read(&path).await {
            Ok(bytes) => {
                let mut resp = bytes.into_response();
                set_content_type(&mut resp, &path);
                Ok(resp)
            }
            Err(error) if error.kind() == ErrorKind::NotFound => {
                Ok(StatusCode::NOT_FOUND.into_response())
            }
            Err(error) => {
                warn!(game = game.name, path = %path.display(), %error, "读取文件时发生错误");
                Ok((StatusCode::INTERNAL_SERVER_ERROR, "读取文件时发生错误").into_response())
            }
        }
    }
}

fn set_content_type(resp: &mut Response, path: impl AsRef<Path>) {
    let mime = mime_guess::from_path(path).first_or_text_plain();
    resp.headers_mut()
        .insert(header::CONTENT_TYPE, mime.essence_str().parse().unwrap());
}

#[instrument(skip_all)]
pub async fn blacklist_layer(State(cfg): State<Cfg>, request: Request, next: Next) -> Response {
    let path = request.uri().path();

    if cfg.server.blacklist.iter().any(|r| r.is_match(path)) {
        debug!(path, "黑名单");
        return (
            StatusCode::NOT_FOUND,
            StatusCode::NOT_FOUND.canonical_reason().unwrap(),
        )
            .into_response();
    }

    next.run(request).await
}
