use crate::{
    common::{self},
    people::common::{insert_person_gender_female, PERSON_GENDERS_URL},
};
use ids_std_rest_testing::{extractors::IntoValueExt, factory::RequestFactory};
use lumx_axum::axum::{body::Body, http::StatusCode};
use lumx_axum_test::program_ext::IntoTestableEndpoints;
use lumx_core::tokio;
use lumx_sea_orm::sea_orm::DatabaseConnection;
use pretty_assertions::assert_eq;
use serde_json::json;
use std::env;
use tower::ServiceExt;

#[tokio::test]
async fn it_not_accept_empty_person_gender_request() {
    let program = common::configure().await;
    let app = program.into_testable_endpoints();

    let req = RequestFactory::post(PERSON_GENDERS_URL, Body::empty());
    let res = app.oneshot(req).await.unwrap();

    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn it_validate_required_person_gender_fields() {
    let program = common::configure().await;
    let app = program.into_testable_endpoints();

    let person_gender_info = json!({
        "name": "",
    });
    let req = RequestFactory::post(
        PERSON_GENDERS_URL,
        Body::from(serde_json::to_string(&person_gender_info).unwrap()),
    );
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);

    let expected_body = json!({
        "errors": [
            {
                "error": "length",
                "field": "name"
            },
        ],
        "message": "validation error"
    });
    assert_eq!(res.into_value().await, expected_body);
}

#[tokio::test]
async fn it_not_accept_duplicate_person_gender() {
    env::set_var("RUST_LOG", "debug");

    let program = common::configure().await;
    let conn = program.get_expect_component::<DatabaseConnection>();
    let app = program.into_testable_endpoints();

    insert_person_gender_female(conn.as_ref()).await.unwrap();

    let person_gender_info = json!({
        "name": "Female"
    });
    let req = RequestFactory::post(
        PERSON_GENDERS_URL,
        Body::from(serde_json::to_string(&person_gender_info).unwrap()),
    );
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::CONFLICT);
}

#[tokio::test]
async fn it_accept_and_save_valid_person_gender() {
    let program = common::configure().await;
    let app = program.into_testable_endpoints();

    let person_gender_info = json!({
        "name": "Female",
        "summary": "Female Description"
    });

    let req = RequestFactory::post(
        PERSON_GENDERS_URL,
        Body::from(serde_json::to_string(&person_gender_info).unwrap()),
    );
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let expected_body = json!({
        "id": 1
    });
    assert_eq!(res.into_value().await, expected_body);
}
