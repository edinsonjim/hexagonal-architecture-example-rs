use crate::domain::changes::product::AddProduct;
use crate::domain::ports::spi::product::ProductRepository;
use crate::domain::selectors::product::ProductSelector;
use ids_std_domain::spi::failure::{SaveRepoFailure, SelectRepoFailure};
use ids_std_sea::convert::into::IntoDomain;
use lumx_sea_orm::sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DbConn, EntityTrait, QueryFilter,
};
use portal_schema::product;
use std::sync::Arc;

#[derive(Clone)]
pub struct ProductSeaRepository {
    db: Arc<DbConn>,
}

impl ProductSeaRepository {
    pub fn new(db: &Arc<DbConn>) -> Self {
        Self {
            db: Arc::clone(&db),
        }
    }
}

#[async_trait::async_trait]
impl ProductRepository for ProductSeaRepository {
    async fn save(&self, event: &AddProduct) -> Result<i32, SaveRepoFailure> {
        let product_model = product::ActiveModel {
            family_id: ActiveValue::Set(event.family_id),
            name: ActiveValue::Set(event.name.to_owned()),
            summary: ActiveValue::Set(event.summary.to_owned()),
            purchasable: ActiveValue::Set(event.purchasable.into()),
            saleable: ActiveValue::Set(event.saleable.into()),
            saleable_without_stock: ActiveValue::Set(event.saleable_without_stock.into()),
            signature: ActiveValue::Set(event.signature.to_owned()),
            ..Default::default()
        };

        product_model
            .save(self.db.as_ref())
            .await
            .map(|model| model.product_id.unwrap())
            .map_err(|err| err.into_domain())
    }

    async fn find_by_signature(
        &self,
        signature: &str,
    ) -> Result<Option<ProductSelector>, SelectRepoFailure> {
        let maybe_model = product::Entity::find()
            .filter(product::Column::Signature.eq(signature))
            .one(self.db.as_ref())
            .await
            .map_err(|err| err.into_domain())?
            .map(ProductSelector::from);

        Ok(maybe_model)
    }
}
