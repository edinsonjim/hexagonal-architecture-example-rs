use lumx_sea_orm::sea_orm::sqlx::types::chrono::Utc;
use lumx_sea_orm::sea_orm::{DatabaseConnection, DbErr};
use portal_migration::sea_orm::{ActiveModelTrait, ActiveValue};
use portal_schema::user;
use serde::Deserialize;

pub const AUTH_URL: &str = "/api/v1/authenticate";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationToken {
    pub access_token: String,
    pub token_type: String,
}

pub async fn insert_user_sample(conn: &DatabaseConnection) -> Result<user::ActiveModel, DbErr> {
    let hashed = bcrypt::hash("idesoftd", bcrypt::DEFAULT_COST).unwrap();

    let user_model = user::ActiveModel {
        username: ActiveValue::Set("idesoftd".to_owned()),
        password: ActiveValue::Set(hashed),
        created_at: ActiveValue::Set(Utc::now().naive_utc()),
        creator_id: ActiveValue::Set(1),
        ..Default::default()
    };

    user_model.save(conn).await
}
