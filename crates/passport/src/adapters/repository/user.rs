use crate::adapters::repository::types::user::UserSelector;
use lumx_sea_orm::sea_orm::QueryFilter;
use lumx_sea_orm::sea_orm::{ColumnTrait, DbConn, EntityTrait};
use passport_core::auth::{FindByUsername, FindByUsernameResult};
use passport_core::user::UserDetails;
use portal_schema::user;
use std::sync::Arc;

#[derive(Clone)]
pub struct UserSeaRepository {
    db: Arc<DbConn>,
}

impl UserSeaRepository {
    pub fn new(db: &Arc<DbConn>) -> Self {
        Self {
            db: Arc::clone(&db),
        }
    }
}

#[async_trait::async_trait]
impl FindByUsername for UserSeaRepository {
    async fn find_by_username(&self, username: String) -> FindByUsernameResult {
        let maybe_model = user::Entity::find()
            .filter(user::Column::Username.eq(username))
            .one(self.db.as_ref())
            .await
            .map_err(|err| {
                tracing::error!(?err, "failed to find user by username");
                passport_core::auth::FindByUsernameFailure::Unknown
            })?;

        let user_details =
            maybe_model.map(|model| Box::new(UserSelector::from(model)) as Box<dyn UserDetails>);

        Ok(user_details)
    }
}
