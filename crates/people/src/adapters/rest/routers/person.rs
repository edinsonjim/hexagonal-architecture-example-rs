use crate::adapters::rest::handlers;
use lumx_axum::axum::{middleware, routing, Router};

pub fn router() -> Router {
    Router::new()
        .route(
            "/api/v1/people",
            routing::post(handlers::person::create_person).get(handlers::person::find_all_people),
        )
        .route(
            "/api/v1/people/:person_id",
            routing::put(handlers::person::update_person).get(handlers::person::find_person),
        )
        .route_layer(middleware::from_fn(
            lumx_passport::middleware::auth::jwt_auth,
        ))
}
