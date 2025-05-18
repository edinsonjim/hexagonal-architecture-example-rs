use crate::domain::queries::person_document_type::PersonDocumentTypeQuery;
use crate::domain::selectors::person_document_type::{
    PersonDocumentTypePageSelector, PersonDocumentTypeSelector,
};
use async_trait::async_trait;
use ids_std_domain::pagination::{Page, PaginationQuery};
use ids_std_domain::spi::failure::SelectRepoFailure;

#[async_trait]
pub trait PersonDocumentTypeRepository: Send + Sync + 'static {
    async fn find_by_id(
        &self,
        id: i32,
    ) -> Result<Option<PersonDocumentTypeSelector>, SelectRepoFailure>;

    async fn find_all(
        &self,
        query: &PaginationQuery,
    ) -> Result<Page<PersonDocumentTypePageSelector>, SelectRepoFailure>;

    async fn find_by_criteria(
        &self,
        query: &PaginationQuery,
        criteria: &PersonDocumentTypeQuery,
    ) -> Result<Page<PersonDocumentTypePageSelector>, SelectRepoFailure>;
}
