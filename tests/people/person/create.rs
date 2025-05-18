use crate::common::{self};
use crate::people::common::{
    insert_document_dni, insert_document_piva, insert_person_gender_female, insert_person_sample_1,
    PEOPLE_URL,
};

use ids_std_rest_testing::extractors::IntoValueExt;
use ids_std_rest_testing::factory::RequestFactory;
use lumx_axum::axum::body::Body;
use lumx_axum::axum::http::StatusCode;
use lumx_axum_test::program_ext::IntoTestableEndpoints;
use lumx_core::tokio;
use lumx_sea_orm::sea_orm::DatabaseConnection;
use pretty_assertions::assert_eq;
use serde_json::json;
use tower::ServiceExt;

#[tokio::test]
async fn it_not_accept_empty_person_request() {
    let program = common::configure().await;
    let app = program.into_testable_endpoints();

    let req = RequestFactory::post(PEOPLE_URL, Body::empty());
    let res = app.oneshot(req).await.unwrap();

    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn it_validate_required_person_fields() {
    let program = common::configure().await;
    let app = program.into_testable_endpoints();

    let person_info = json!({
        "firstName": "",
        "lastName": "",
        "documentNumber": "",
        "documentTypeId": 1,
        "genderId": 1
    });

    let req = RequestFactory::post(
        PEOPLE_URL,
        Body::from(serde_json::to_string(&person_info).unwrap()),
    );
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);

    let expected_body = json!({
        "errors": [
            {
                "error": "length",
                "field": "document_number"
            },
            {
                "error": "length",
                "field": "first_name"
            },
            {
                "error": "length",
                "field": "last_name"
            },
        ],
        "message": "validation error"
    });
    assert_eq!(res.into_value().await, expected_body);
}

#[tokio::test]
async fn it_not_accept_invalid_document_type() {
    let program = common::configure().await;
    let app = program.into_testable_endpoints();

    let person_info = json!({
        "firstName": "Idesoft",
        "lastName": "Systems",
        "documentNumber": "ID3SOFT",
        "documentTypeId": 1,
        "genderId": 1
    });

    let req = RequestFactory::post(
        PEOPLE_URL,
        Body::from(serde_json::to_string(&person_info).unwrap()),
    );
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);

    let expected_body = json!({
        "errors": [
            {
                "error": "document type does not exist",
                "field": "document_type_id",
            }
        ],
        "message": "validation error"
    });
    assert_eq!(res.into_value().await, expected_body);
}

#[tokio::test]
async fn it_not_accept_invalid_gender() {
    let program = common::configure().await;
    let conn = program.get_expect_component::<DatabaseConnection>();
    let app = program.into_testable_endpoints();

    insert_document_dni(conn.as_ref()).await.unwrap();

    let person_info = json!({
        "firstName": "Idesoft",
        "lastName": "Systems",
        "documentNumber": "ID3SOFT",
        "documentTypeId": 1,
        "genderId": 1
    });

    let req = RequestFactory::post(
        PEOPLE_URL,
        Body::from(serde_json::to_string(&person_info).unwrap()),
    );
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);

    let expected_body = json!({
        "errors": [
            {
                "field": "gender_id",
                "error": "gender does not exist"
            }
        ],
        "message": "validation error"
    });
    assert_eq!(res.into_value().await, expected_body);
}

#[tokio::test]
async fn it_not_accept_duplicate_person() {
    let program = common::configure().await;
    let conn = program.get_expect_component::<DatabaseConnection>();
    let app = program.into_testable_endpoints();

    insert_person_sample_1(conn.as_ref()).await;

    let person_info = json!({
        "firstName": "Idesoft",
        "lastName": "Systems",
        "documentNumber": "ID3SOFT",
        "documentTypeId": 1,
        "genderId": 1
    });

    let req = RequestFactory::post(
        PEOPLE_URL,
        Body::from(serde_json::to_string(&person_info).unwrap()),
    );
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::CONFLICT);
}

#[tokio::test]
async fn it_accept_and_save_valid_person() {
    let program = common::configure().await;
    let conn = program.get_expect_component::<DatabaseConnection>();
    let app = program.into_testable_endpoints();

    insert_document_piva(conn.as_ref()).await.unwrap();
    insert_person_gender_female(conn.as_ref()).await.unwrap();

    let person_info = json!({
        "firstName": "Idesoft",
        "lastName": "Systems",
        "documentNumber": "ID3SOFT",
        "documentTypeId": 1,
        "genderId": 1
    });

    let req = RequestFactory::post(
        PEOPLE_URL,
        Body::from(serde_json::to_string(&person_info).unwrap()),
    );
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let expected_body = json!({
        "id": 1
    });
    assert_eq!(res.into_value().await, expected_body);
}
