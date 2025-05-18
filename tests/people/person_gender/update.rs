use ids_std_rest_testing::{extractors::IntoValueExt, factory::RequestFactory};
use lumx_axum::axum::{body::Body, http::StatusCode};
use lumx_axum_test::program_ext::IntoTestableEndpoints;
use lumx_core::tokio;
use lumx_sea_orm::sea_orm::{DatabaseConnection, EntityTrait};
use portal_schema::person_gender;
use serde_json::json;
use tower::ServiceExt;

use crate::{
    common::{self},
    people::common::{insert_person_gender_female, insert_person_gender_male, PERSON_GENDERS_URL},
};

fn url_to_update_person_gender(person_gender_id: i32) -> String {
    format!("{PERSON_GENDERS_URL}/{person_gender_id}")
}

#[tokio::test]
async fn it_not_accept_empty_person_gender_request() {
    let program = common::configure().await;
    let app = program.into_testable_endpoints();

    let req = RequestFactory::put(url_to_update_person_gender(1).as_str(), Body::empty());
    let res = app.oneshot(req).await.unwrap();

    assert_eq!(res.status(), StatusCode::BAD_REQUEST)
}

#[tokio::test]
async fn it_validate_required_person_gender_fields_to_update() {
    let program = common::configure().await;
    let app = program.into_testable_endpoints();

    let update_person_gender = json!({
        "name": ""
    });

    let req = RequestFactory::put(
        url_to_update_person_gender(1).as_str(),
        Body::from(serde_json::to_string(&update_person_gender).unwrap()),
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
    assert_eq!(res.into_value().await, expected_body)
}

#[tokio::test]
async fn it_not_accept_duplicate_gender_signature() {
    let program = common::configure().await;
    let conn = program.get_expect_component::<DatabaseConnection>();
    let app = program.into_testable_endpoints();

    insert_person_gender_male(conn.as_ref()).await.unwrap();
    insert_person_gender_female(conn.as_ref()).await.unwrap();

    let person_gender_info = json!({
        "name": "Female"
    });
    let req = RequestFactory::put(
        url_to_update_person_gender(1).as_str(),
        Body::from(serde_json::to_string(&person_gender_info).unwrap()),
    );
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::CONFLICT);
}

#[tokio::test]
async fn it_accept_and_update_valid_person_gender() {
    let program = common::configure().await;
    let conn = program.get_expect_component::<DatabaseConnection>();
    let app = program.into_testable_endpoints();

    insert_person_gender_male(conn.as_ref()).await.unwrap();

    let person_gender_id_to_update: i32 = 1;
    let person_gender_info = json!({
        "name": "Masculino",
        "summary": "Masculino"
    });
    let req = RequestFactory::put(
        url_to_update_person_gender(person_gender_id_to_update).as_str(),
        Body::from(serde_json::to_string(&person_gender_info).unwrap()),
    );
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::NO_CONTENT);

    let updated_person_gender = person_gender::Entity::find_by_id(person_gender_id_to_update)
        .into_json()
        .one(conn.as_ref())
        .await
        .unwrap()
        .unwrap();

    let expected_person_gender = json!({
        "person_gender_id": person_gender_id_to_update,
        "name": "Masculino",
        "summary": "Masculino",
        "signature": "b699db5848174056f1b149a8ebe01201"
    });
    assert_eq!(expected_person_gender, updated_person_gender);
}

#[tokio::test]
async fn it_accept_and_update_no_required_fields() {
    let program = common::configure().await;
    let conn = program.get_expect_component::<DatabaseConnection>();
    let app = program.into_testable_endpoints();

    insert_person_gender_male(conn.as_ref()).await.unwrap();

    let person_gender_id_to_update: i32 = 1;
    let person_gender_info = json!({
        "name": "Male",
        "summary": "Masculino"
    });
    let req = RequestFactory::put(
        url_to_update_person_gender(person_gender_id_to_update).as_str(),
        Body::from(serde_json::to_string(&person_gender_info).unwrap()),
    );
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::NO_CONTENT);

    let updated_person_gender = person_gender::Entity::find_by_id(person_gender_id_to_update)
        .into_json()
        .one(conn.as_ref())
        .await
        .unwrap()
        .unwrap();

    let expected_person_gender = json!({
        "person_gender_id": person_gender_id_to_update,
        "name": "Male",
        "summary": "Masculino",
        "signature": "07cf4f8f5d8b76282917320715dda2ad"
    });
    assert_eq!(expected_person_gender, updated_person_gender);
}
