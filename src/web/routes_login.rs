use axum::{Json, Router, routing::post};
use serde::Deserialize;
use serde_json::{Value, json};
use tower_cookies::{Cookie, Cookies};

use crate::{Error, Result};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");

    // TODO: Implement real db/auth logic.
    if payload.username != "admin" || payload.pwd != "admin" {
        return Err(Error::LoginFail);
    }

    // FIXME: Implement real auth-token generation/signature.
    cookies.add(Cookie::new("auth_token", "user-1.exp.sign"));

    // Crate the success body
    let body = Json(json!({
        "status": "success",
        "message": "Login successful"
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}
