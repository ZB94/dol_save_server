use axum::{
    Json, Router,
    extract::{Request, State},
    http::{HeaderValue, StatusCode},
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
    routing::{get, post},
};
use tower_http::set_header::SetResponseHeaderLayer;
use tower_sessions::Session;

use crate::Cfg;

pub mod auth;
pub mod pwa;
pub mod save;

pub fn route() -> Router<Cfg> {
    Router::new()
        // 在线检查
        .route("/alive", get(auth::alive))
        // 保存存档/存档列表
        .route("/save", post(save::save).get(save::list))
        // 获取/删除存档
        .route("/save/{name}", get(save::code).delete(save::remove))
        // 登录接口
        .route("/login", post(auth::login))
        // PWA 是否启用接口
        .route("/pwa/enabled", get(pwa::enabled))
        // 所有接口请求禁用缓存
        .layer(SetResponseHeaderLayer::overriding(
            axum::http::header::CACHE_CONTROL,
            HeaderValue::from_static("no-store"),
        ))
}

#[instrument(skip_all)]
pub async fn auth_layer(
    State(cfg): State<Cfg>,
    session: Session,
    mut request: Request,
    next: Next,
) -> Response {
    const WHITE_LIST: &[&str] = &[
        // 登录相关
        "/api/login",
        "/login.html",
        // PWA 相关
        "sw.js",
        "/pwa/icon.png",
        "/pwa/manifest.json",
    ];
    const API_PREFIX: &str = "/api/";

    let user = session.get::<String>(User::KEY).await.unwrap_or_default();

    let path = request.uri().path();
    let is_global = cfg.auth.global;
    let is_api = path.starts_with(API_PREFIX);

    debug!(path, is_global, is_api, ?user, "auth");

    if !cfg.auth.enable || WHITE_LIST.contains(&path) || (!is_global && !is_api) || user.is_some() {
        debug!(uri = %request.uri(), "鉴权通过");

        if !is_global || is_api {
            request
                .extensions_mut()
                .insert(User(user.unwrap_or_default()));
        }

        return next.run(request).await;
    }

    if is_api {
        (StatusCode::UNAUTHORIZED, Json("需要登录")).into_response()
    } else {
        Redirect::temporary("/login.html").into_response()
    }
}

#[derive(Debug, Clone)]
pub struct User(pub String);

impl User {
    const KEY: &str = "user";

    pub async fn set_session(
        self,
        session: &Session,
    ) -> Result<(), tower_sessions::session::Error> {
        session.insert(Self::KEY, self.0).await
    }
}
