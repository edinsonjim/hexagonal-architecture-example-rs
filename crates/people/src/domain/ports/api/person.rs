use async_trait::async_trait;
use ids_std_domain::api::failure::{
    CreateDomainFailure, FindManyFailure, FindOneFailure, UpdateDomainFailure,
};
use ids_std_domain::pagination::{Page, PaginationQuery};

use crate::domain::commands::person::{CreatePersonCommand, UpdatePersonCommand};
use crate::domain::selectors::person::{PersonDetailsSelector, PersonPageSelector};

#[async_trait]
pub trait CreatePersonUseCase: Send + Sync + 'static {
    async fn create(&self, person: &CreatePersonCommand) -> Result<i32, CreateDomainFailure>;
}

#[async_trait]
pub trait FindAllPeopleUseCase: Send + Sync + 'static {
    async fn find_all_people(
        &self,
        query: &PaginationQuery,
    ) -> Result<Page<PersonPageSelector>, FindManyFailure>;
}

#[async_trait]
pub trait UpdatePersonUseCase: Send + Sync + 'static {
    async fn update_person(&self, command: &UpdatePersonCommand)
        -> Result<(), UpdateDomainFailure>;
}

#[async_trait]
pub trait FindPersonDetailsUseCase: Send + Sync + 'static {
    async fn find_person_details(
        &self,
        person_id: i32,
    ) -> Result<PersonDetailsSelector, FindOneFailure>;
}
