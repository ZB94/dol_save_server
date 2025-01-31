use std::collections::HashMap;

use axum::{
    extract::{FromRequest, Request, State},
    http::Method,
    response::{Html, IntoResponse, Redirect, Response},
    Form,
};
use serde::Deserialize;
use tower_sessions::Session;

pub async fn login(
    State(state): State<crate::State>,
    session: Session,
    request: Request,
) -> Response {
    const HTML: &str = include_str!("../../html/login.html");
    let mut msg = String::default();

    if request.method() == Method::POST {
        if let Ok(Form(user)) = Form::<User>::from_request(request, &()).await {
            debug!(?user, "用户登入");

            // 获取用户信息
            let u = if state.auth_file.exists() {
                tokio::fs::read_to_string(&state.auth_file)
                    .await
                    .inspect_err(|error| error!(%error, "读取用户列表失败"))
                    .ok()
                    .and_then(|s| {
                        serde_json::from_str::<HashMap<String, String>>(&s)
                            .inspect_err(|error| error!(%error, "解析用户列表失败"))
                            .ok()
                    })
                    .and_then(|mut users| users.remove_entry(&user.username))
                    .map(|(username, password)| User { username, password })
            } else {
                let default_user = Default::default();
                error!(?default_user, "用户列表不存在, 使用默认用户");
                Some(default_user)
            };

            // 校验用户名和密码
            if u.is_some_and(|u| u == user)
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

    Html::<String>(HTML.replace("{message}", &msg).into()).into_response()
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
struct User {
    pub username: String,
    pub password: String,
}

impl Default for User {
    fn default() -> Self {
        Self {
            username: "anonymous".to_string(),
            password: Default::default(),
        }
    }
}
