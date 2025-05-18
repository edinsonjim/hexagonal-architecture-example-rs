use ids_std_rest_testing::{extractors::IntoValueExt, factory::RequestFactory};
use lumx_axum::axum::http::StatusCode;
use lumx_axum_test::program_ext::IntoTestableEndpoints;
use lumx_core::tokio;
use lumx_sea_orm::sea_orm::DatabaseConnection;
use pretty_assertions::assert_eq;
use serde_json::json;
use tower::ServiceExt;

use crate::{
    common::{self},
    people::common::{insert_people_sample_to_paginate, insert_person_sample_1, PEOPLE_URL},
};

#[tokio::test]
async fn it_retrieve_empty_paginate_people() {
    let program = common::configure().await;
    let app = program.into_testable_endpoints();

    let req = RequestFactory::get(format!("{PEOPLE_URL}?page=1&page_size=10").as_str());
    let res = app.oneshot(req).await.unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    let expected_body = json!({
        "data": [],
        "total": 0,
        "page": 0,
        "pageSize": 10,
    });
    assert_eq!(res.into_value().await, expected_body)
}

#[tokio::test]
async fn it_retrieve_paginate_people() {
    let program = common::configure().await;
    let conn = program.get_expect_component::<DatabaseConnection>();
    let app = program.into_testable_endpoints();

    insert_person_sample_1(conn.as_ref()).await;

    let req = RequestFactory::get(format!("{PEOPLE_URL}?page=1&page_size=10").as_str());
    let res = app.oneshot(req).await.unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    let expected_body = json!({
        "data": [
            {
                "personId": 1,
                "firstName": "Idesoft",
                "lastName": "Systems",
                "fullName": "Idesoft Systems",
                "documentNumber": "ID3SOFT",
                "documentTypeId": 1,
                "documentTypeName": "P.IVA",
                "genderId": 1,
                "genderName": "Female"
            }
        ],
        "total": 1,
        "page": 1,
        "pageSize": 10
    });
    assert_eq!(res.into_value().await, expected_body)
}

#[tokio::test]
async fn it_retrieve_paginate_second_page_people() {
    let program = common::configure().await;
    let conn = program.get_expect_component::<DatabaseConnection>();
    let app = program.into_testable_endpoints();

    insert_people_sample_to_paginate(conn.as_ref()).await;

    let req = RequestFactory::get(format!("{PEOPLE_URL}?page=2&page_size=10").as_str());
    let res = app.oneshot(req).await.unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    let expected_body = json!({
        "data": [
            {
                "personId": 3,
                "firstName": "Person",
                "lastName": "Three",
                "fullName": "Person Three",
                "documentNumber": "0003",
                "documentTypeId": 1,
                "documentTypeName": "P.IVA",
                "genderId": 1,
                "genderName": "Female"
            },
            {
                "personId": 2,
                "firstName": "Person",
                "lastName": "Two",
                "fullName": "Person Two",
                "documentNumber": "0002",
                "documentTypeId": 1,
                "documentTypeName": "P.IVA",
                "genderId": 2,
                "genderName": "Male"
            },
            {
                "personId": 1,
                "firstName": "Person",
                "lastName": "One",
                "fullName": "Person One",
                "documentNumber": "0001",
                "documentTypeId": 1,
                "documentTypeName": "P.IVA",
                "genderId": 1,
                "genderName": "Female"
            }
        ],
        "total": 13,
        "page": 2,
        "pageSize": 10
    });
    assert_eq!(res.into_value().await, expected_body)
}
