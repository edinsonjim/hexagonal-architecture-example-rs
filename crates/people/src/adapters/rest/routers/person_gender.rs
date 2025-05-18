use lumx_axum::axum::{routing, Router};

use crate::adapters::rest::handlers;

pub fn router() -> Router {
    Router::new()
        .route(
            "/api/v1/person-genders",
            routing::post(handlers::person_gender::create_person_gender),
        )
        .route(
            "/api/v1/person-genders/:person_gender_id",
            routing::put(handlers::person_gender::update_person_gender),
        )
}
