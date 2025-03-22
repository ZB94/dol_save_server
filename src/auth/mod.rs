mod login;

use axum::{
    extract::{FromRequestParts, Request},
    http::{request::Parts, StatusCode},
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
    routing::get,
    Router,
};

use tower::ServiceBuilder;

pub use login::login;
use tower_sessions::{Expiry, MemoryStore, Session, SessionManagerLayer};

use crate::Cfg;

pub async fn router(router: Router<Cfg>, secure: bool) -> Router<Cfg> {
    let store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(store)
        .with_secure(secure)
        .with_expiry(Expiry::OnSessionEnd);

    router
        .route("/api/alive", get("OK"))
        .route("/login", get(login).post(login))
        .layer(
            ServiceBuilder::new()
                .layer(session_layer)
                .layer(axum::middleware::from_fn(verify)),
        )
}

async fn verify(session: Session, request: Request, next: Next) -> Response {
    trace!(?session, "鉴权");

    if session
        .get::<String>("user")
        .await
        .is_ok_and(|v| v.is_some())
        || request.uri().path() == "/login"
    {
        return next.run(request).await;
    }

    Redirect::to("/login").into_response()
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

impl FromRequestParts<Cfg> for User {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &Cfg) -> Result<Self, Self::Rejection> {
        if state.auth.enable {
            let session = Session::from_request_parts(parts, state).await?;
            match session.get::<String>(Self::KEY).await {
                Ok(Some(user)) => Ok(User(user)),
                _ => Err((StatusCode::UNAUTHORIZED, "登入验证未通过")),
            }
        } else {
            Ok(User(Default::default()))
        }
    }
}
