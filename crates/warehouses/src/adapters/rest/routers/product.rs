use crate::adapters::rest::handlers;
use lumx_axum::axum::{routing, Router};

pub fn router() -> Router {
    Router::new().route(
        "/api/v1/products",
        routing::post(handlers::product::create_product),
    )
}
