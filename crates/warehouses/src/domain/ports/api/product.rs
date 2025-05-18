use crate::domain::commands::product::CreateProductCommand;
use async_trait::async_trait;
use ids_std_domain::api::failure::CreateDomainFailure;

#[async_trait]
pub trait CreateProductUseCase: Send + Sync + 'static {
    async fn create_product(
        &self,
        command: &CreateProductCommand,
    ) -> Result<i32, CreateDomainFailure>;
}
