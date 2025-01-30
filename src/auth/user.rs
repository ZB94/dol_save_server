use async_trait::async_trait;
use axum_session_auth::Authentication;

#[derive(Debug, Clone)]
pub struct User;

#[async_trait]
impl Authentication<User, String, ()> for User {
    async fn load_user(_userid: String, _pool: Option<&()>) -> anyhow::Result<User> {
        Ok(User)
    }

    fn is_authenticated(&self) -> bool {
        true
    }

    fn is_active(&self) -> bool {
        true
    }

    fn is_anonymous(&self) -> bool {
        false
    }
}
