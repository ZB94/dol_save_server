mod login;
mod user;

use std::{path::PathBuf, sync::Arc};

use axum::{
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
    routing::get,
    Extension, Router,
};
use axum_session::SessionLayer;
use axum_session_auth::{
    AuthConfig, AuthSession, AuthSessionLayer, SessionNullPool, SessionNullSessionStore,
};
use tower::ServiceBuilder;

pub use login::login;
pub use user::User;

pub async fn router(user_path: PathBuf, router: Router) -> Router {
    let store = SessionNullSessionStore::new(None, Default::default())
        .await
        .unwrap();

    let auth_config = AuthConfig::<String>::default();

    router.route("/login", get(login).post(login)).layer(
        ServiceBuilder::new()
            .layer(Extension(UserPath(Arc::new(user_path))))
            .layer(SessionLayer::new(store))
            .layer(
                AuthSessionLayer::<User, String, SessionNullPool, ()>::new(None)
                    .with_config(auth_config),
            )
            .layer(axum::middleware::from_fn(verify)),
    )
}

#[derive(Debug, Clone)]
pub(crate) struct UserPath(pub Arc<PathBuf>);

async fn verify(
    session: AuthSession<User, String, SessionNullPool, ()>,
    Extension(UserPath(_path)): Extension<UserPath>,
    request: Request,
    next: Next,
) -> Response {
    trace!(?session, "鉴权");

    if session.is_authenticated()
        || request.uri().path() == "/api/login"
        || request.uri().path() == "/login"
    {
        return next.run(request).await;
    }

    Redirect::to("/login").into_response()
}
