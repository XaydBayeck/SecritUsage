use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use sqlx::SqlitePool;
use tower_cookies::{Cookie, Cookies};

use super::{Login, Model, Selector, UserLogin};

const COOKIENAME: &str = "login_user";

pub async fn login(
    cookies: Cookies,
    State(pool): State<Arc<SqlitePool>>,
    Json(user): Json<UserLogin>,
) -> impl IntoResponse {
    let selector = match user.login {
        Login::Name(name) => Selector::Name(name),
        Login::Email(email) => Selector::Email(email),
    };

    let user = Model::select_by(selector, &pool)
        .await
        .map_err(|e| (StatusCode::NOT_FOUND, e.to_string()))?;

    if cookies.get(COOKIENAME).is_some() {
        cookies.remove(Cookie::new(COOKIENAME, ""));
    }
    let user_json = serde_json::to_string(&user).expect("User Model can't serialize correctly!");

    cookies.add(Cookie::new(COOKIENAME, user_json));
    Ok::<(), (StatusCode, String)>(())
}
