use axum::{http::StatusCode, response::IntoResponse, response::Response};
use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, Serialize, strum_macros::AsRefStr)]
#[serde(tag="type", content = "data")]
pub enum Error {
    LoginFail,
    TicketDeletionFailtureIdNotfound { id: u64 },
    AuthFailNoAuthTokenCookie,
    AuthFailWrongTokenFormat,
    AuthFailCTXNotInRequest,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");

        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        response.extensions_mut().insert(self).unwrap();

        return response;
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::LoginFail => write!(f, "Login failed"),
            Error::TicketDeletionFailtureIdNotfound { id } => write!(f, "Ticket deletion failed"),
            Error::AuthFailNoAuthTokenCookie => write!(f, "No Auth Token Present in Request"),
            Error::AuthFailWrongTokenFormat => write!(f, "Auth Token has wrong format"),
            Error::AuthFailCTXNotInRequest => write!(f, "Auth Context not in request"),
        }
    }
}

impl std::error::Error for Error {}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        #[allow(unreachable_patterns)]
        match self {
            Error::LoginFail => (StatusCode::UNAUTHORIZED, ClientError::LOGIN_FAIL),

            Error::TicketDeletionFailtureIdNotfound { id: _ } => {
                (StatusCode::BAD_REQUEST, ClientError::INVALD_PARAMS)
            }

            Error::AuthFailNoAuthTokenCookie
            | Error::AuthFailWrongTokenFormat
            | Error::AuthFailCTXNotInRequest => (StatusCode::UNAUTHORIZED, ClientError::NO_AUTH),

            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALD_PARAMS,
    SERVICE_ERROR,
}
