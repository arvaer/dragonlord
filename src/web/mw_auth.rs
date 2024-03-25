use crate::crypt::token::{validate_web_token, Token};
use crate::ctx::Ctx;
use crate::model::user::{UserBMC, UserForAuth};
use crate::model::ModelManager;
use crate::utils::b64u_decode;
use crate::web::{set_token_cookie, AUTH_TOKEN};
use crate::web::{Error, Result};
use async_trait::async_trait;
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use serde::Serialize;
use tower_cookies::{Cookie, Cookies};
use tracing::debug;

#[allow(dead_code)] // For now, until we have the rpc.
pub async fn mw_ctx_require<B>(
    ctx: Result<Ctx>,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    debug!("{:<12} - mw_ctx_require - {ctx:?}", "MIDDLEWARE");

    ctx?;

    Ok(next.run(req).await)
}

pub async fn mw_ctx_resolve<B>(
    mm: State<ModelManager>,
    cookies: Cookies,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    debug!("{:<12} - mw_ctx_resolve", "MIDDLEWARE");

    let result_ctx = _ctx_resolve(mm, &cookies).await;
    if result_ctx.is_err() && !matches!(result_ctx, Err(CtxExtError::TokenNotInCookie)){
        cookies.remove(Cookie::named(AUTH_TOKEN))
    }
    // Store the ctx_result in the request extension.
    req.extensions_mut().insert(result_ctx);

    Ok(next.run(req).await)
}

async fn _ctx_resolve(mm: State<ModelManager>, cookies: &Cookies) -> CtxExtResult {
    let token = cookies
        .get(AUTH_TOKEN)
        .map(|c| c.value().to_string())
        .ok_or(CtxExtError::TokenNotInCookie)?;

    let token = token
        .parse::<Token>()
        .map_err(|_| CtxExtError::TokenWrongFormat)?;

    let ctx = Ctx::root_ctx();
    let user: UserForAuth = UserBMC::first_by_username(&ctx, &mm, &token.ident)
        .await
        .map_err(|e| CtxExtError::ModelAccessError(e.to_string()))?
        .ok_or(CtxExtError::UserNotFound)?;

    validate_web_token(&token, &user.token_salt.to_string())
        .map_err(|_| CtxExtError::FailedTokenValidation)?;

    set_token_cookie(cookies, &user.username, &user.token_salt.to_string())
        .map_err(|_| CtxExtError::FailedToUpdateCookie)?;

    Ctx::new(user.id).map_err(|ex|CtxExtError::CtxCreateFail(ex.to_string()))


}

// region:    --- Ctx Extractor
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        debug!("{:<12} - Ctx", "EXTRACTOR");

        parts
            .extensions
            .get::<CtxExtResult>()
            .ok_or(Error::CtxExt(CtxExtError::CtxNotInRequestExt))?
            .clone()
            .map_err(Error::CtxExt)
    }
}
// endregion: --- Ctx Extractor

// region:    --- Ctx Extractor Result/Error
type CtxExtResult = core::result::Result<Ctx, CtxExtError>;

#[derive(Clone, Serialize, Debug)]
pub enum CtxExtError {
    TokenNotInCookie,
    TokenWrongFormat,
    CtxNotInRequestExt,
    CtxCreateFail(String),
    ModelAccessError(String),
    UserNotFound,
    FailedTokenValidation,
    FailedToUpdateCookie,
}
// endregion: --- Ctx Extractor Result/Error
