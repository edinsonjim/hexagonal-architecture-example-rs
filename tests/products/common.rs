use lumx_sea_orm::sea_orm;
use lumx_sea_orm::sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection};
use portal_schema::{product, product_family};
use warehouses::domain::valuables::product::ProductSignature;

pub const PRODUCTS_URL: &str = "/api/v1/products";

pub struct ProductFactory;

impl ProductFactory {
    pub async fn iphone_x(
        conn: &DatabaseConnection,
    ) -> Result<product::ActiveModel, sea_orm::DbErr> {
        let family = ProductFamilyFactory::electronics(conn).await?;

        let product_name = "iPhone X";
        let product_signature = ProductSignature::new(product_name).get();

        let product_model = product::ActiveModel {
            family_id: ActiveValue::Set(family.product_family_id.unwrap()),
            name: ActiveValue::Set(product_name.to_string()),
            summary: ActiveValue::Set(Some("iPhone 10".to_string())),
            purchasable: ActiveValue::Set(true.into()),
            saleable: ActiveValue::Set(true.into()),
            saleable_without_stock: ActiveValue::Set(false.into()),
            signature: ActiveValue::Set(product_signature.to_owned()),
            ..Default::default()
        };

        product_model.save(conn).await
    }
}

pub struct ProductFamilyFactory;

impl ProductFamilyFactory {
    pub async fn electronics(
        conn: &DatabaseConnection,
    ) -> Result<product_family::ActiveModel, sea_orm::DbErr> {
        let family_model = product_family::ActiveModel {
            name: ActiveValue::Set("Electronics".to_owned()),
            summary: ActiveValue::Set(Some("All Electronics".to_string())),
            signature: ActiveValue::Set("".to_owned()),
            ..Default::default()
        };

        family_model.save(conn).await
    }
}
