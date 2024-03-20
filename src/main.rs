#![allow(unused)]
pub use self::error::{Error, Result};
use crate::model::ModelController;
use axum::Json;
use axum::extract::Query;
use axum::http::{Uri, Method};
use axum::response::IntoResponse;
use axum::routing::{get, get_service};
use axum::{middleware, response::Response, Router};
use ctx::Ctx;
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_cookies::*;
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};
use tower_http::services::ServeDir;
use axum::http::Request;

mod error;
mod model;
mod web;
mod ctx;
mod log;

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

fn routes_hello() -> Router {
    Router::new().route("/hello", get(handler_hello))
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("/")))
}

#[tokio::main]
async fn main() -> Result<()>{
    let mc = ModelController::new().await?;

    let routes_apis = web::routes_tickets::routes(mc.clone()).route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let all_routes = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", routes_apis)
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn_with_state(mc.clone(), web::mw_auth::ctx_resolver))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("->> LISTENING on {:?}\n", listener.local_addr());
    axum::serve(listener, all_routes.into_make_service())
        .await
        .unwrap();
    Ok(())
}

async fn main_response_mapper(ctx: Option<Ctx>, uri: Uri, req_method: Method, res: Response) -> Response {
    println!("->> {:<12} - {res:?}", "RES_MAPPEr");
    let uuid = Uuid::new_v4();

    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|e| e.client_status_and_error());
    let error_response =
        client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let body = json!({
                "error": {
                    "type": client_error.as_ref(),
                    "req_uuid": uuid.to_string(),
                }
            });
            println!(" --> error_response - {body:?}",);
            (*status_code, Json(body)).into_response()
        });


    println!("->> {:<12} - {error_response:?}", "RES_MAPPER_ERROR");
    // server log
    log::log_request(uuid, uri, req_method, ctx, client_status_error.unzip().1, service_error).await;
    println!();

    error_response.unwrap_or(res)

}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("handler_hello - {params:?}");
    let name = params.name.as_deref().unwrap_or("World");
    return format!("Hello, {}!", name);
}
