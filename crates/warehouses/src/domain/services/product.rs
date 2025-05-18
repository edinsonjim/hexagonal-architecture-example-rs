use crate::domain::changes::product::AddProduct;
use crate::domain::commands::product::CreateProductCommand;
use crate::domain::ports::api::product::CreateProductUseCase;
use crate::domain::ports::spi::family::ProductFamilyRepository;
use crate::domain::ports::spi::product::ProductRepository;
use crate::domain::valuables::product::ProductSignature;
use ids_std_domain::api::failure::{CreateDomainFailure, InvalidField};
use ids_std_domain::validation;
use std::sync::Arc;

#[derive(Clone)]
pub struct ProductService {
    product_repo: Arc<dyn ProductRepository>,
    family_repo: Arc<dyn ProductFamilyRepository>,
}

impl ProductService {
    pub fn new(
        product_repo: Arc<dyn ProductRepository>,
        family_repo: Arc<dyn ProductFamilyRepository>,
    ) -> Self {
        Self {
            product_repo,
            family_repo,
        }
    }
}

#[async_trait::async_trait]
impl CreateProductUseCase for ProductService {
    async fn create_product(
        &self,
        command: &CreateProductCommand,
    ) -> Result<i32, CreateDomainFailure> {
        tracing::info!("creating product {:?}", command);

        validation::Validator::try_validate(command)?;

        let product_family = self.family_repo.find_by_id(command.family_id).await?;

        if product_family.is_none() {
            tracing::info!(
                product_family_id = &command.family_id,
                "product family does not exist"
            );

            Err(CreateDomainFailure::InvalidField(InvalidField::new(
                "product_family_id".into(),
                "product family does not exist".into(),
            )))?
        }

        let product_signature = ProductSignature::new(command.name.as_str()).get();

        let another_similar_product = self
            .product_repo
            .find_by_signature(product_signature.as_str())
            .await?;

        if let Some(prd) = another_similar_product {
            tracing::info!(
                product_signature = &prd.signature,
                "product signature already exist"
            );

            Err(CreateDomainFailure::Conflict(
                "product signature already exist".to_string(),
            ))?;
        }

        let product_created_event = AddProduct {
            name: command.name.to_owned(),
            summary: command.summary.to_owned(),
            family_id: command.family_id,
            purchasable: command.purchasable,
            saleable: command.saleable,
            saleable_without_stock: command.saleable_without_stock,
            signature: product_signature.to_owned(),
        };
        let product_id = self.product_repo.save(&product_created_event).await?;

        Ok(product_id)
    }
}
