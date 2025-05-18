use crate::common::{self};
use crate::people::common::{
    PEOPLE_URL,
    {
        insert_document_dni, insert_person_gender_male, insert_person_sample_1,
        insert_person_sample_2,
    },
};
use ids_std_rest_testing::extractors::IntoValueExt;
use ids_std_rest_testing::factory::RequestFactory;
use lumx_axum::axum::body::Body;
use lumx_axum::axum::http::StatusCode;
use lumx_axum_test::program_ext::IntoTestableEndpoints;
use lumx_core::tokio;
use lumx_sea_orm::sea_orm::{DatabaseConnection, EntityTrait};
use portal_schema::person;
use pretty_assertions::assert_eq;
use serde_json::json;
use tower::ServiceExt;

fn url_to_update_person(person_id: i32) -> String {
    format!("{PEOPLE_URL}/{person_id}")
}

#[tokio::test]
async fn it_not_accept_empty_person_request() {
    let program = common::configure().await;
    let app = program.into_testable_endpoints();

    let req = RequestFactory::put(url_to_update_person(1).as_str(), Body::empty());
    let res = app.oneshot(req).await.unwrap();

    assert_eq!(res.status(), StatusCode::BAD_REQUEST)
}

#[tokio::test]
async fn it_validate_required_person_fields_to_update() {
    let program = common::configure().await;
    let app = program.into_testable_endpoints();

    let update_person = json!({
        "firstName": "",
        "lastName": "",
        "documentNumber": "",
        "documentTypeId": 1,
        "genderId": 1
    });

    let req = RequestFactory::put(
        url_to_update_person(1).as_str(),
        Body::from(serde_json::to_string(&update_person).unwrap()),
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
    assert_eq!(res.into_value().await, expected_body)
}

#[tokio::test]
async fn it_not_accept_invalid_person_id() {
    let program = common::configure().await;
    let app = program.into_testable_endpoints();

    let person_id: i32 = 10;
    let update_person = json!({
        "firstName": "Idesoft",
        "lastName": "Systems",
        "documentNumber": "ID3SOFT",
        "documentTypeId": 1,
        "genderId": 1
    });

    let req = RequestFactory::put(
        url_to_update_person(person_id).as_str(),
        Body::from(serde_json::to_string(&update_person).unwrap()),
    );
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);

    let expected_body = json!({
        "errors": [
            {
                "field": "person_id",
                "error": "person does not exist"
            }
        ],
        "message": "validation error"
    });
    assert_eq!(res.into_value().await, expected_body)
}

#[tokio::test]
async fn it_not_accept_invalid_document_type() {
    let program = common::configure().await;
    let conn = program.get_expect_component::<DatabaseConnection>();
    let app = program.into_testable_endpoints();

    insert_person_sample_1(conn.as_ref()).await;

    let update_person = json!({
        "firstName": "Idesoft",
        "lastName": "Systems",
        "documentNumber": "ID3SOFT",
        "documentTypeId": 10,
        "genderId": 1
    });

    let req = RequestFactory::put(
        url_to_update_person(1).as_str(),
        Body::from(serde_json::to_string(&update_person).unwrap()),
    );
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);

    let expected_body = json!({
        "errors": [
            {
                "field": "document_type_id",
                "error": "document type does not exist"
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

    insert_person_sample_1(conn.as_ref()).await;

    let update_person = json!({
        "firstName": "Idesoft",
        "lastName": "Systems",
        "documentNumber": "ID3SOFT",
        "documentTypeId": 1,
        "genderId": 20
    });

    let req = RequestFactory::put(
        url_to_update_person(1).as_str(),
        Body::from(serde_json::to_string(&update_person).unwrap()),
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
async fn it_not_accept_duplicate_person_signature() {
    let program = common::configure().await;
    let conn = program.get_expect_component::<DatabaseConnection>();
    let app = program.into_testable_endpoints();

    insert_person_sample_1(conn.as_ref()).await;

    let update_person = json!({
        "firstName": "iDesoft",
        "lastName": "sYstEms",
        "documentNumber": "ID3SOFT",
        "documentTypeId": 1,
        "genderId": 1
    });

    let req = RequestFactory::put(
        url_to_update_person(1).as_str(),
        Body::from(serde_json::to_string(&update_person).unwrap()),
    );
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::CONFLICT)
}

#[tokio::test]
async fn it_not_accept_duplicate_document() {
    let program = common::configure().await;
    let conn = program.get_expect_component::<DatabaseConnection>();
    let app = program.into_testable_endpoints();

    insert_person_sample_1(&conn).await;
    insert_person_sample_2(&conn).await;

    let update_person = json!({
        "firstName": "Idesoft",
        "lastName": "Systems",
        "documentNumber": "0001",
        "documentTypeId": 2,
        "genderId": 1
    });

    let req = RequestFactory::put(
        url_to_update_person(1).as_str(),
        Body::from(serde_json::to_string(&update_person).unwrap()),
    );
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::CONFLICT)
}

#[tokio::test]
async fn it_accept_and_update_valid_person() {
    let program = common::configure().await;
    let conn = program.get_expect_component::<DatabaseConnection>();
    let app = program.into_testable_endpoints();

    insert_person_sample_1(conn.as_ref()).await;
    let another_doc = insert_document_dni(conn.as_ref()).await.unwrap();
    let another_doc_id = another_doc.person_document_type_id.unwrap();

    let another_gender = insert_person_gender_male(conn.as_ref()).await.unwrap();
    let another_gender_id = another_gender.person_gender_id.unwrap();

    let person_id_to_update: i32 = 1;
    let update_person = json!({
        "firstName": "Idesoft",
        "lastName": "Systems Tech",
        "documentNumber": "0001",
        "documentTypeId": another_doc_id,
        "genderId": another_gender_id,
    });

    let req = RequestFactory::put(
        url_to_update_person(person_id_to_update).as_str(),
        Body::from(serde_json::to_string(&update_person).unwrap()),
    );
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::NO_CONTENT);

    let updated_person = person::Entity::find_by_id(person_id_to_update)
        .into_json()
        .one(conn.as_ref())
        .await
        .unwrap()
        .unwrap();

    let expected_person = json!({
        "person_id": person_id_to_update,
        "first_name": "Idesoft",
        "last_name": "Systems Tech",
        "document_number": "0001",
        "document_type_id": another_doc_id,
        "gender_id": another_gender_id,
        "signature": "548696ad63cc5a7b6c86e4c1b39c89b6"
    });

    assert_eq!(expected_person, updated_person)
}
