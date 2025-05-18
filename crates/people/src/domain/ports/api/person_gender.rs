use async_trait::async_trait;
use ids_std_domain::{
    api::failure::{CreateDomainFailure, FindManyFailure, UpdateDomainFailure},
    pagination::{Page, PaginationQuery},
};

use crate::domain::{
    commands::person_gender::{CreatePersonGenderCommand, UpdatePersonGenderCommand},
    queries::person_gender::PersonGenderQuery,
    selectors::person_gender::PersonGenderPageSelector,
};

#[async_trait]
pub trait FindPersonGenderByCriteriaUseCase: Send + Sync + 'static {
    async fn find_person_gender_by_criteria(
        &self,
        query: &PaginationQuery,
        criteria: &PersonGenderQuery,
    ) -> Result<Page<PersonGenderPageSelector>, FindManyFailure>;
}

#[async_trait]
pub trait CreatePersonGenderUseCase: Send + Sync + 'static {
    async fn create(&self, cmd: &CreatePersonGenderCommand) -> Result<i32, CreateDomainFailure>;
}

#[async_trait]
pub trait UpdatePersonGenderUseCase: Send + Sync + 'static {
    async fn update_person_gender(
        &self,
        command: &UpdatePersonGenderCommand,
    ) -> Result<(), UpdateDomainFailure>;
}
