use async_trait::async_trait;
use ids_std_domain::{
    api::failure::FindManyFailure,
    pagination::{Page, PaginationQuery},
};

use crate::domain::{
    queries::person_document_type::PersonDocumentTypeQuery,
    selectors::person_document_type::PersonDocumentTypePageSelector,
};

#[async_trait]
pub trait FindAllPersonDocumentTypeUseCase: Send + Sync + 'static {
    async fn find_all_person_document_type(
        &self,
        query: &PaginationQuery,
    ) -> Result<Page<PersonDocumentTypePageSelector>, FindManyFailure>;
}

#[async_trait]
pub trait FindPersonDocumentTypeByCriteriaUseCase: Send + Sync + 'static {
    async fn find_person_document_type_by_criteria(
        &self,
        query: &PaginationQuery,
        criteria: &PersonDocumentTypeQuery,
    ) -> Result<Page<PersonDocumentTypePageSelector>, FindManyFailure>;
}
