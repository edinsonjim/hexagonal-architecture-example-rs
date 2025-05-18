use crate::{
    common::{self},
    people::common::{insert_person_sample_1, PEOPLE_URL},
};
use ids_std_rest_testing::{extractors::IntoValueExt, factory::RequestFactory};
use lumx_axum::axum::http::StatusCode;
use lumx_axum_test::program_ext::IntoTestableEndpoints;
use lumx_core::tokio;
use lumx_sea_orm::sea_orm::DatabaseConnection;
use pretty_assertions::assert_eq;
use serde_json::json;
use tower::ServiceExt;

fn url_to_retrieve_person(person_id: i32) -> String {
    format!("{PEOPLE_URL}/{person_id}")
}

#[tokio::test]
async fn it_not_accept_invalid_person_id() {
    let program = common::configure().await;
    let app = program.into_testable_endpoints();

    let person_id: i32 = 10;
    let req = RequestFactory::get(url_to_retrieve_person(person_id).as_str());
    let res = app.oneshot(req).await.unwrap();

    assert_eq!(res.status(), StatusCode::NOT_FOUND);

    let expected_body = json!({
        "errors": [],
        "message": "person does not exist"
    });
    assert_eq!(res.into_value().await, expected_body)
}

#[tokio::test]
async fn it_retrieve_person_details() {
    let program = common::configure().await;
    let conn = program.get_expect_component::<DatabaseConnection>();
    let app = program.into_testable_endpoints();

    insert_person_sample_1(conn.as_ref()).await;

    let person_id = 1;
    let req = RequestFactory::get(url_to_retrieve_person(person_id).as_str());
    let res = app.oneshot(req).await.unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    let expected_body = json!({
        "personId": 1,
        "firstName": "Idesoft",
        "lastName": "Systems",
        "fullName": "Idesoft Systems",
        "documentNumber": "ID3SOFT",
        "documentTypeId": 1,
        "documentTypeName": "P.IVA",
        "genderId": 1,
        "genderName": "Female"
    });
    assert_eq!(res.into_value().await, expected_body)
}
