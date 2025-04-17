use std::fmt;

use axum::{Json, extract::State, http::StatusCode};
use serde::Deserialize;
use tower_sessions::Session;

use crate::Cfg;

pub async fn login(
    State(state): State<Cfg>,
    session: Session,
    Json(user): Json<User>,
) -> Result<&'static str, (StatusCode, &'static str)> {
    debug!(?user, "用户登入");

    // 获取用户信息
    let u = if state.auth.users.is_empty() {
        warn!("当前用户列表为空");
        None
    } else {
        state
            .auth
            .users
            .iter()
            .find(|u| u.username == user.username)
    };

    // 校验用户名和密码
    if u.is_some_and(|u| &user == u)
        && super::User(user.username.clone())
            .set_session(&session)
            .await
            .is_ok()
    {
        Ok("登录成功")
    } else {
        const MSG: &str = "用户名或密码错误";
        warn!(?user, "{MSG}");
        Err((StatusCode::BAD_REQUEST, MSG))
    }
}

#[derive(Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
}

impl PartialEq<crate::config::User> for User {
    fn eq(&self, other: &crate::config::User) -> bool {
        self.username == other.username && self.password == other.password
    }
}

impl fmt::Debug for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("User")
            .field("username", &self.username)
            .field("password", &"***")
            .finish()
    }
}
