use crate::products::common::{ProductFactory, ProductFamilyFactory};
use crate::{
    common::{self},
    products::common::PRODUCTS_URL,
};
use ids_std_rest_testing::extractors::IntoValueExt;
use ids_std_rest_testing::factory::RequestFactory;
use lumx_axum::axum::{body::Body, http::StatusCode};
use lumx_axum_test::program_ext::IntoTestableEndpoints;
use lumx_core::tokio;
use lumx_sea_orm::sea_orm::{DatabaseConnection, EntityTrait};
use portal_schema::product;
use serde_json::json;
use tower::ServiceExt;

#[tokio::test]
async fn it_not_accept_empty_product_request() {
    let program = common::configure().await;
    let app = program.into_testable_endpoints();

    let req = RequestFactory::post(PRODUCTS_URL, Body::empty());
    let res = app.oneshot(req).await.unwrap();

    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn it_validate_required_product_fields() {
    let program = common::configure().await;
    let app = program.into_testable_endpoints();

    let product_info = json!({
        "name": "",
        "familyId": 1,
        "purchasable": true,
        "saleable": true,
        "saleableWithoutStock": true
    });
    let req = RequestFactory::post(
        PRODUCTS_URL,
        Body::from(serde_json::to_string(&product_info).unwrap()),
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
async fn it_not_accept_invalid_family_type() {
    let program = common::configure().await;
    let app = program.into_testable_endpoints();

    let product_info = json!({
        "name": "iPhone 100",
        "familyId": 1,
        "purchasable": true,
        "saleable": true,
        "saleableWithoutStock": true
    });
    let req = RequestFactory::post(
        PRODUCTS_URL,
        Body::from(serde_json::to_string(&product_info).unwrap()),
    );
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);

    let expected_body = json!({
        "errors": [
            {
                "field": "product_family_id",
                "error": "product family does not exist"
            }
        ],
        "message": "validation error"
    });
    assert_eq!(res.into_value().await, expected_body);
}

#[tokio::test]
async fn it_not_accept_duplicate_product() {
    let program = common::configure().await;
    let conn = program.get_expect_component::<DatabaseConnection>();
    let app = program.into_testable_endpoints();

    ProductFactory::iphone_x(conn.as_ref()).await.unwrap();

    let product_info = json!({
        "name": "iPhone X",
        "familyId": 1,
        "purchasable": true,
        "saleable": true,
        "saleableWithoutStock": true
    });
    let req = RequestFactory::post(
        PRODUCTS_URL,
        Body::from(serde_json::to_string(&product_info).unwrap()),
    );
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::CONFLICT);
}

#[tokio::test]
async fn it_accepts_and_save_valid_product() {
    let program = common::configure().await;
    let conn = program.get_expect_component::<DatabaseConnection>();
    let app = program.into_testable_endpoints();

    ProductFamilyFactory::electronics(conn.as_ref())
        .await
        .unwrap();

    let product_info = json!({
        "name": "iPhone X",
        "summary": "iPhone 100",
        "familyId": 1,
        "purchasable": true,
        "saleable": true,
        "saleableWithoutStock": false
    });
    let req = RequestFactory::post(
        PRODUCTS_URL,
        Body::from(serde_json::to_string(&product_info).unwrap()),
    );
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let expected_body = json!({
        "id": 1
    });
    assert_eq!(res.into_value().await, expected_body);

    let product_saved = product::Entity::find_by_id(1)
        .into_json()
        .one(conn.as_ref())
        .await
        .unwrap()
        .unwrap();

    let expected_product = json!({
        "product_id": 1,
        "name": "iPhone X",
        "summary": "iPhone 100",
        "family_id": 1,
        "purchasable": true,
        "saleable": true,
        "saleable_without_stock": false,
        "signature": "882a0465d260983ada874710ef46aaef"
    });
    assert_eq!(product_saved, expected_product);
}
