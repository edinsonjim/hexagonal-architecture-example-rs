use crate::adapters::rest::handlers;
use lumx_axum::axum::{routing, Router};

pub fn router() -> Router {
    Router::new().route(
        "/api/v1/authenticate",
        routing::post(handlers::auth::authenticate),
    )
}
