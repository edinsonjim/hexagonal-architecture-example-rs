use std::sync::Arc;

use crate::domain::{
    ports::{
        api::person_document_type::{
            FindAllPersonDocumentTypeUseCase, FindPersonDocumentTypeByCriteriaUseCase,
        },
        spi::person_document_type::PersonDocumentTypeRepository,
    },
    queries::person_document_type::PersonDocumentTypeQuery,
    selectors::person_document_type::PersonDocumentTypePageSelector,
};
use async_trait::async_trait;
use ids_std_domain::{
    api::failure::FindManyFailure,
    pagination::{Page, PaginationQuery},
};

#[derive(Clone)]
pub struct PersonDocumentTypeService {
    person_document_type_repo: Arc<dyn PersonDocumentTypeRepository>,
}

impl PersonDocumentTypeService {
    pub fn new(person_document_type_repo: Arc<dyn PersonDocumentTypeRepository>) -> Self {
        Self {
            person_document_type_repo,
        }
    }
}

#[async_trait]
impl FindAllPersonDocumentTypeUseCase for PersonDocumentTypeService {
    async fn find_all_person_document_type(
        &self,
        query: &PaginationQuery,
    ) -> Result<Page<PersonDocumentTypePageSelector>, FindManyFailure> {
        Ok(self.person_document_type_repo.find_all(query).await?)
    }
}

#[async_trait]
impl FindPersonDocumentTypeByCriteriaUseCase for PersonDocumentTypeService {
    async fn find_person_document_type_by_criteria(
        &self,
        query: &PaginationQuery,
        criteria: &PersonDocumentTypeQuery,
    ) -> Result<Page<PersonDocumentTypePageSelector>, FindManyFailure> {
        Ok(self
            .person_document_type_repo
            .find_by_criteria(query, criteria)
            .await?)
    }
}
