use std::time::SystemTime;

use crate::{Error, Result, error::ClientError, ctx::Ctx};
use axum::http::{Uri, Method, Request};
use serde::Serialize;
use serde_json::{json, Value};
use serde_with::skip_serializing_none;
use uuid::Uuid;


#[skip_serializing_none]
#[derive(Serialize)]
struct RequestLogLine {
    uuid: String,
    timestamp: String,

    user_id: Option<u64>,
    req_path: String,
    req_method: String,

    client_error_type: Option<String>,
    error_type: Option<String>,
    error_data: Option<Value>
}



pub async fn log_request(uuid: Uuid, uri: Uri, req_method: Method, ctx: Option<Ctx>, client_error: Option<ClientError>, service_error: Option<&Error>) -> Result<()> {

    let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();
    let error_type = service_error.map(|se| se.as_ref().to_string());
    let error_data = serde_json::to_value(service_error).ok().and_then(|mut v| v.get_mut("data").map(|v| v.take()));

    let log_line = RequestLogLine {
        uuid: uuid.to_string(),
        timestamp: timestamp.to_string(),

        user_id: ctx.map(|c| c.user_id()),
        req_path: uri.to_string(),
        req_method: req_method.to_string(),

        client_error_type: client_error.map(|e| e.as_ref().to_string()),
        error_type,
        error_data
    };

    let log_line = json!(log_line);
    println!("{}", log_line.to_string());
    Ok(())
}