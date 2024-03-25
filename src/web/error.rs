use crate::crypt;
use crate::model;
use crate::web;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use tracing::debug;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    // -- Login
    LoginFail,
    LoginFailUsernameNotFound,
    LoginFailNoPassword { user_id: i64 },
    LoginFailPasswordMismatch { user_id: i64 },
    LogoutFail,
    // -- CtxExtError
    CtxExt(web::mw_auth::CtxExtError),
    // -- RPC
    RpcMethodUnknown(String),
    RpcMissingParams {
        rpc_method: String
    },
    RpcFailToInterpret{
        rpc_method: String
    },
    // Modules
    Model(model::Error),
    Crypt(crypt::Error),
    // Extern Modules
    Serde(String)
}

// region:    --- Axum IntoResponse
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        debug!("{:<12} - model::Error {self:?}", "INTO_RES");

        // Create a placeholder Axum reponse.
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // Insert the Error into the reponse.
        response.extensions_mut().insert(self);

        response
    }
}
// endregion: --- Axum IntoResponse

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl From<model::Error> for Error {
    fn from(val: model::Error) -> Self {
        Self::Model(val)
    }
}

impl From<crypt::Error> for Error {
    fn from(val: crypt::Error) -> Self{
        Self::Crypt(val)
    }
}
 impl From<serde_json::Error> for Error {
     fn from(val: serde_json::Error) -> Self{
         Self::Serde(val.to_string())
     }
 }

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate

// region:    --- Client Error

/// From the root error to the http status code and ClientError
impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        use web::Error::*;

        #[allow(unreachable_patterns)]
        match self {
            // -- Login
            LoginFailUsernameNotFound
            | LoginFail
            | LoginFailPasswordMismatch { user_id: _ }
            | LoginFailNoPassword { user_id: _ } => {
                (StatusCode::UNAUTHORIZED, ClientError::LOGIN_FAIL)
            }

            // --Auth
            CtxExt(_) => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),

            // --Model
            Model(model::Error::EntityNotFound { entity, id}) => (
                StatusCode::BAD_REQUEST,
                ClientError::ENTITY_NOT_FOUND{ entity, id: *id }
            ),

            // -- Fallback.
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}

#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag="message", content="detail")]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    SERVICE_ERROR,
    ENTITY_NOT_FOUND { entity: &'static str, id: i64}
}
// endregion: --- Client Error
