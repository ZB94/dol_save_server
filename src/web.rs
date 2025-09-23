use std::convert::Infallible;

use axum::{
    extract::{Request, State},
    http::{StatusCode, header},
    middleware::Next,
    response::{IntoResponse, Response},
};
use include_dir::{Dir, include_dir};

use crate::Cfg;

const WEB: Dir = include_dir!("web");

#[instrument(skip_all)]
pub async fn web_service(request: Request) -> Result<Response, Infallible> {
    let path = request.uri().path();
    debug!(path, "web request path");

    let path = path.trim_start_matches('/');

    if let Some(file) = WEB.get_file(path) {
        let mut resp = file.contents().into_response();
        let mime = mime_guess::from_path(path).first_or_text_plain();
        resp.headers_mut()
            .insert(header::CONTENT_TYPE, mime.essence_str().parse().unwrap());

        Ok(resp)
    } else {
        Ok(StatusCode::NOT_FOUND.into_response())
    }
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
