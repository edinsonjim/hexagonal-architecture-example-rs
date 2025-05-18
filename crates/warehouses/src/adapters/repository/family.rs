use crate::domain::ports::spi::family::ProductFamilyRepository;
use crate::domain::selectors::family::ProductFamilySelector;
use ids_std_domain::spi::failure::SelectRepoFailure;
use ids_std_sea::convert::into::IntoDomain;
use lumx_sea_orm::sea_orm::{DbConn, EntityTrait};
use portal_schema::product_family;
use std::sync::Arc;

#[derive(Clone)]
pub struct ProductFamilySeaRepository {
    db: Arc<DbConn>,
}

impl ProductFamilySeaRepository {
    pub fn new(db: &Arc<DbConn>) -> Self {
        Self {
            db: Arc::clone(&db),
        }
    }
}

#[async_trait::async_trait]
impl ProductFamilyRepository for ProductFamilySeaRepository {
    async fn find_by_id(
        &self,
        id: i32,
    ) -> Result<Option<ProductFamilySelector>, SelectRepoFailure> {
        let maybe_model = product_family::Entity::find_by_id(id)
            .one(self.db.as_ref())
            .await
            .map_err(|err| err.into_domain())?
            .map(ProductFamilySelector::from);

        Ok(maybe_model)
    }
}
