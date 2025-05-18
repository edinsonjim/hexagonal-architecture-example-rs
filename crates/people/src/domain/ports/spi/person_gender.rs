use crate::domain::changes::person_gender::{AddPersonGender, UpdatePersonGender};
use crate::domain::queries::person_gender::PersonGenderQuery;
use crate::domain::selectors::person_gender::{PersonGenderPageSelector, PersonGenderSelector};
use async_trait::async_trait;
use ids_std_domain::pagination::{Page, PaginationQuery};
use ids_std_domain::spi::failure::{SaveRepoFailure, SelectRepoFailure};

#[async_trait]
pub trait PersonGenderRepository: Send + Sync + 'static {
    async fn find_by_id(&self, id: i32) -> Result<Option<PersonGenderSelector>, SelectRepoFailure>;

    async fn find_by_criteria(
        &self,
        query: &PaginationQuery,
        criteria: &PersonGenderQuery,
    ) -> Result<Page<PersonGenderPageSelector>, SelectRepoFailure>;

    async fn find_by_signature(
        &self,
        signature: &str,
    ) -> Result<Option<PersonGenderSelector>, SelectRepoFailure>;

    async fn find_other_by_signature(
        &self,
        self_id: i32,
        signature: &str,
    ) -> Result<Option<PersonGenderSelector>, SelectRepoFailure>;

    async fn save(&self, event: AddPersonGender) -> Result<i32, SaveRepoFailure>;

    async fn update(&self, event: &UpdatePersonGender) -> Result<(), SaveRepoFailure>;
}
