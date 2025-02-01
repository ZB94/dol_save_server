use std::fmt;

use axum::{
    extract::{FromRequest, Request, State},
    http::Method,
    response::{Html, IntoResponse, Redirect, Response},
    Form,
};
use serde::Deserialize;
use tower_sessions::Session;

use crate::Cfg;

pub async fn login(State(state): State<Cfg>, session: Session, request: Request) -> Response {
    const HTML: &str = include_str!("../../html/login.html");
    let mut msg = String::default();

    if request.method() == Method::POST {
        if let Ok(Form(user)) = Form::<User>::from_request(request, &()).await {
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
                return Redirect::to("/").into_response();
            } else {
                warn!(?user, "用户名或密码错误");
                msg = "用户名或密码错误".to_string();
            }
        }
    }

    Html(HTML.replace("{message}", &msg)).into_response()
}

#[derive(Deserialize)]
struct User {
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
