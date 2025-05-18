use crate::common;
use crate::passport::common::{insert_user_sample, AuthenticationToken, AUTH_URL};
use ids_std_rest_testing::extractors::IntoValueExt;
use ids_std_rest_testing::factory::RequestFactory;
use lumx_axum::axum::body::Body;
use lumx_axum::axum::http::StatusCode;
use lumx_axum_test::program_ext::IntoTestableEndpoints;
use lumx_core::tokio;
use lumx_sea_orm::sea_orm::DatabaseConnection;
use passport_core::decoder::DecodeAccessToken;
use passport_jwt::decoder::AccessTokenDecoder;
use serde_json::json;
use tower::ServiceExt;

#[tokio::test]
async fn it_not_accept_empty_authenticate_request() {
    let program = common::configure().await;
    let app = program.into_testable_endpoints();

    let req = RequestFactory::post(AUTH_URL, Body::empty());
    let res = app.oneshot(req).await.unwrap();

    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn it_not_authorize_empty_credentials() {
    let program = common::configure().await;
    let app = program.into_testable_endpoints();

    let credentials = json!({
        "username": "",
        "password": ""
    });

    let req = RequestFactory::post(
        AUTH_URL,
        Body::from(serde_json::to_string(&credentials).unwrap()),
    );
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn it_not_authorize_unknown_users() {
    let program = common::configure().await;
    let app = program.into_testable_endpoints();

    let credentials = json!({
        "username": "bluebirdbot",
        "password": "bluebird"
    });

    let req = RequestFactory::post(
        AUTH_URL,
        Body::from(serde_json::to_string(&credentials).unwrap()),
    );
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn it_not_authorize_invalid_credentials() {
    let program = common::configure().await;
    let conn = program.get_expect_component::<DatabaseConnection>();
    let app = program.into_testable_endpoints();

    insert_user_sample(conn.as_ref()).await.unwrap();

    let credentials = json!({
        "username": "idesoftd",
        "password": "bluebird"
    });

    let req = RequestFactory::post(
        AUTH_URL,
        Body::from(serde_json::to_string(&credentials).unwrap()),
    );
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn it_authorize_and_check_access_token() {
    let program = common::configure().await;
    let conn = program.get_expect_component::<DatabaseConnection>();
    let token_decoder = program.get_expect_component::<AccessTokenDecoder>();
    let app = program.into_testable_endpoints();

    let user_model = insert_user_sample(conn.as_ref()).await.unwrap();

    let credentials = json!({
        "username": "idesoftd",
        "password": "idesoftd"
    });
    let req = RequestFactory::post(
        AUTH_URL,
        Body::from(serde_json::to_string(&credentials).unwrap()),
    );
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let token: AuthenticationToken = serde_json::from_value(res.into_value().await).unwrap();

    let user_claims = token_decoder
        .decode_access_token(token.access_token)
        .await
        .unwrap();

    assert_eq!(token.token_type, "Bearer");
    assert_eq!(user_claims.sub(), user_model.username.unwrap());
    assert_eq!(user_claims.sub_id(), user_model.id.unwrap());
    assert_eq!(user_claims.iss(), "auth.portal.idesoft.co");
    assert_eq!(user_claims.aud(), "co.idesoft.portal");
}
