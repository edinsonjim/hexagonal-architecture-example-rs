use crate::domain::selectors::family::ProductFamilySelector;
use ids_std_domain::spi::failure::SelectRepoFailure;

#[async_trait::async_trait]
pub trait ProductFamilyRepository: Send + Sync + 'static {
    async fn find_by_id(&self, id: i32)
        -> Result<Option<ProductFamilySelector>, SelectRepoFailure>;
}
