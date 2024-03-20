use crate::{Error, Result, web};
use axum::Json;
use serde::Deserialize;
use serde_json::{Value, json};
use axum::Router;
use axum::routing::post;
use tower_cookies::{Cookie, Cookies};


pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("api_login - {:?}", payload);

    if payload.username == "admin" && payload.password == "admin" {
        cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));
        Ok(Json(json!({ "status": "ok" })))
    } else {
        Err(Error::LoginFail)
    }
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}
