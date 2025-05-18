use crate::domain::changes::product::AddProduct;
use crate::domain::selectors::product::ProductSelector;
use ids_std_domain::spi::failure::{SaveRepoFailure, SelectRepoFailure};

#[async_trait::async_trait]
pub trait ProductRepository: Send + Sync + 'static {
    async fn save(&self, event: &AddProduct) -> Result<i32, SaveRepoFailure>;

    async fn find_by_signature(
        &self,
        signature: &str,
    ) -> Result<Option<ProductSelector>, SelectRepoFailure>;
}
