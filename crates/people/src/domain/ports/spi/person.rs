use crate::domain::entities::person::PersonEnt;
use crate::domain::selectors::person::{PersonDetailsSelector, PersonPageSelector, PersonSelector};
use async_trait::async_trait;
use ids_std_domain::pagination::{Page, PaginationQuery};
use ids_std_domain::spi::failure::{SaveRepoFailure, SelectRepoFailure};

#[async_trait]
pub trait PersonRepository: Send + Sync + 'static {
    async fn save(&self, person: &PersonEnt) -> Result<i32, SaveRepoFailure>;

    async fn update(&self, person: &PersonEnt) -> Result<(), SaveRepoFailure>;

    async fn find_by_id(&self, id: i32) -> Result<Option<PersonSelector>, SelectRepoFailure>;

    async fn find_details_by_id(
        &self,
        id: i32,
    ) -> Result<Option<PersonDetailsSelector>, SelectRepoFailure>;

    async fn find_by_signature(
        &self,
        signature: &str,
    ) -> Result<Option<PersonSelector>, SelectRepoFailure>;

    async fn find_by_document_and_type(
        &self,
        document_number: &str,
        document_type_id: i32,
    ) -> Result<Option<PersonSelector>, SelectRepoFailure>;

    async fn find_another_by_document_and_type(
        &self,
        self_id: i32,
        document_number: &str,
        document_type_id: i32,
    ) -> Result<Option<PersonSelector>, SelectRepoFailure>;

    async fn find_all(
        &self,
        query: &PaginationQuery,
    ) -> Result<Page<PersonPageSelector>, SelectRepoFailure>;
}
