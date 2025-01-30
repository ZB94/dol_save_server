mod login;

use std::{path::PathBuf, sync::Arc};

use axum::{
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
    routing::get,
    Extension, Router,
};

use tower::ServiceBuilder;

pub use login::login;
use tower_sessions::{Expiry, MemoryStore, Session, SessionManagerLayer};

pub async fn router(user_path: PathBuf, router: Router) -> Router {
    let store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(store)
        .with_secure(false)
        .with_expiry(Expiry::OnSessionEnd);

    router
        .route("/api/alive", get("OK"))
        .route("/login", get(login).post(login))
        .layer(
            ServiceBuilder::new()
                .layer(Extension(UserPath(Arc::new(user_path))))
                .layer(session_layer)
                .layer(axum::middleware::from_fn(verify)),
        )
}

#[derive(Debug, Clone)]
pub(crate) struct UserPath(pub Arc<PathBuf>);

async fn verify(
    session: Session,
    Extension(UserPath(_path)): Extension<UserPath>,
    request: Request,
    next: Next,
) -> Response {
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
