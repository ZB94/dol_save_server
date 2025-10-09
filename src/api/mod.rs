use axum::{
    Json, Router,
    extract::{Request, State},
    http::{HeaderValue, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, set_header::SetResponseHeaderLayer};
use tower_sessions::Session;

use crate::Cfg;

pub mod auth;
pub mod pwa;
pub mod save;

pub fn route(cfg: Cfg) -> Router<Cfg> {
    Router::new()
        // 在线检查
        .route("/alive", get(auth::alive))
        // 保存存档/存档列表
        .route("/save", post(save::save).get(save::list))
        // 获取/删除存档
        .route("/save/{name}", get(save::code).delete(save::remove))
        .layer(
            ServiceBuilder::new()
                .layer(axum::middleware::from_fn_with_state(
                    cfg.clone(),
                    auth_layer,
                ))
                .layer(axum::middleware::from_fn_with_state(
                    cfg.clone(),
                    save::layer_game_name,
                )),
        )
        // 登录接口
        .route("/login", post(auth::login))
        // PWA 是否启用接口
        .route("/pwa/enabled", get(pwa::enabled))
        // 所有接口请求禁用缓存
        .layer(
            ServiceBuilder::new()
                .layer(SetResponseHeaderLayer::overriding(
                    axum::http::header::CACHE_CONTROL,
                    HeaderValue::from_static("no-store"),
                ))
                .option_layer(cfg.server.cors.then(CorsLayer::very_permissive)),
        )
}

#[instrument(skip_all)]
async fn auth_layer(
    State(cfg): State<Cfg>,
    session: Session,
    mut request: Request,
    next: Next,
) -> Response {
    let user = session
        .get::<String>(User::SESSION_KEY)
        .await
        .unwrap_or_default();

    if !cfg.auth.enable || user.is_some() {
        debug!(uri = %request.uri(), "鉴权通过");

        request
            .extensions_mut()
            .insert(User(user.unwrap_or_default()));

        return next.run(request).await;
    }

    (StatusCode::UNAUTHORIZED, Json("需要登录")).into_response()
}

#[derive(Debug, Clone)]
pub struct User(pub String);

impl User {
    pub const SESSION_KEY: &str = "user";

    pub async fn set_session(
        self,
        session: &Session,
    ) -> Result<(), tower_sessions::session::Error> {
        session.insert(Self::SESSION_KEY, self.0).await
    }
}
