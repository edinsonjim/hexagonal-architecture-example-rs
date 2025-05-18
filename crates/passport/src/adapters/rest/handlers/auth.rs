use crate::adapters::rest::types::UsernamePasswordAuthenticationParams;
use ids_std_rest_api::{failure::ApiFailure, replier::Replier, types::result::ApiResult};
use lumx_axum::axum::Json;
use lumx_axum::extractor::Component;
use passport_jwt::auth::TokenAuthManager;
use passport_jwt::{TokenUsernamePasswordAuth, TokenUsernamePasswordAuthFailure};
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationToken {
    pub access_token: String,
    pub token_type: String,
}

pub async fn authenticate(
    Component(manager): Component<TokenAuthManager>,
    Json(payload): Json<UsernamePasswordAuthenticationParams>,
) -> ApiResult<AuthenticationToken> {
    tracing::info!(username = payload.username, "authenticating username");

    let authentication = manager
        .authenticate(payload.username, payload.password)
        .await
        .map_err(|err| match err {
            TokenUsernamePasswordAuthFailure::BadCredentials => {
                ApiFailure::Unauthorized(err.to_string())
            }
            TokenUsernamePasswordAuthFailure::AccessDenied => {
                ApiFailure::Forbidden(err.to_string())
            }
            TokenUsernamePasswordAuthFailure::Unknown => {
                tracing::error!(?err, "failed to authenticate credentials");
                ApiFailure::Unknown(err.to_string())
            }
        })?;

    Ok(Replier::ok(AuthenticationToken {
        access_token: authentication.access_token.to_owned(),
        token_type: authentication.token_type.to_owned(),
    }))
}
