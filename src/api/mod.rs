use axum::{
    Json, Router,
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use tower::ServiceBuilder;
use tower_sessions::{Expiry, MemoryStore, Session, SessionManagerLayer};

use crate::Cfg;

pub mod auth;
pub mod pwa;
pub mod save;

pub fn route(cfg: Cfg) -> Router<Cfg> {
    let store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(store)
        .with_secure(cfg.tls.enable)
        .with_expiry(Expiry::OnSessionEnd);

    Router::new()
        // 在线检查
        .route("/alive", get(auth::alive))
        // 登录接口
        .route("/login", post(auth::login))
        // 保存存档/存档列表
        .route("/save", post(save::save).get(save::list))
        // 获取/删除存档
        .route("/save/{name}", get(save::code).delete(save::remove))
        // 权限校验中间件
        .layer(
            ServiceBuilder::new()
                .layer(session_layer)
                .layer(axum::middleware::from_fn_with_state(cfg, auth_layer)),
        )
        .route("/pwa/enabled", get(pwa::enabled))
}

#[instrument(skip_all)]
async fn auth_layer(
    State(cfg): State<Cfg>,
    session: Session,
    mut request: Request,
    next: Next,
) -> Response {
    const WHITE_LIST: &[&str] = &["/login"];

    debug!(uri = %request.uri(), "auth");

    let user = session.get::<String>(User::KEY).await.unwrap_or_default();

    if !cfg.auth.enable || WHITE_LIST.contains(&request.uri().path()) || user.is_some() {
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
    const KEY: &str = "user";

    pub async fn set_session(
        self,
        session: &Session,
    ) -> Result<(), tower_sessions::session::Error> {
        session.insert(Self::KEY, self.0).await
    }
}
