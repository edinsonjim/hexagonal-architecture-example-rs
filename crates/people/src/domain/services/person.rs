use std::sync::Arc;

use crate::domain::commands::person::{CreatePersonCommand, UpdatePersonCommand};
use crate::domain::entities::person::PersonEnt;
use crate::domain::ports::api::person::{
    CreatePersonUseCase, FindAllPeopleUseCase, FindPersonDetailsUseCase, UpdatePersonUseCase,
};
use crate::domain::ports::spi::person::PersonRepository;
use crate::domain::ports::spi::person_document_type::PersonDocumentTypeRepository;
use crate::domain::ports::spi::person_gender::PersonGenderRepository;
use crate::domain::selectors::person::{PersonDetailsSelector, PersonPageSelector};
use async_trait::async_trait;
use ids_std_domain::api::failure::{
    CreateDomainFailure, FindManyFailure, FindOneFailure, InvalidField, UpdateDomainFailure,
};
use ids_std_domain::pagination::{Page, PaginationQuery};

#[derive(Clone)]
pub struct PersonService {
    person_repo: Arc<dyn PersonRepository>,
    person_document_type_repo: Arc<dyn PersonDocumentTypeRepository>,
    person_gender_repo: Arc<dyn PersonGenderRepository>,
}

impl PersonService {
    pub fn new(
        person_repo: Arc<dyn PersonRepository>,
        person_document_type_repo: Arc<dyn PersonDocumentTypeRepository>,
        person_gender_repo: Arc<dyn PersonGenderRepository>,
    ) -> Self {
        Self {
            person_repo,
            person_document_type_repo,
            person_gender_repo,
        }
    }
}

#[async_trait]
impl CreatePersonUseCase for PersonService {
    async fn create(&self, person: &CreatePersonCommand) -> Result<i32, CreateDomainFailure> {
        tracing::info!("creating person {:?}", person);

        let person_ent = PersonEnt::try_from(person)?;

        let document_type = self
            .person_document_type_repo
            .find_by_id(person_ent.document_type_id)
            .await?;

        if document_type.is_none() {
            tracing::info!(
                document_type_id = &person_ent.document_type_id,
                "document type does not exist"
            );

            return Err(CreateDomainFailure::InvalidField(InvalidField::new(
                "document_type_id".into(),
                "document type does not exist".into(),
            )));
        }

        let person_gender = self
            .person_gender_repo
            .find_by_id(person_ent.gender_id)
            .await?;
        if person_gender.is_none() {
            tracing::info!(gender_id = &person_ent.gender_id, "gender does not exist");

            return Err(CreateDomainFailure::InvalidField(InvalidField::new(
                "gender_id".into(),
                "gender does not exist".to_string(),
            )));
        }

        let person_selector = self
            .person_repo
            .find_by_signature(&person_ent.signature)
            .await?;

        if let Some(c) = person_selector {
            tracing::info!(signature = &c.signature, "person signature already exist");

            return Err(CreateDomainFailure::Conflict(
                "person with signature already exist".to_string(),
            ));
        }

        let person_selector = self
            .person_repo
            .find_by_document_and_type(&person_ent.document_number, person_ent.document_type_id)
            .await?;

        if let Some(p) = person_selector {
            tracing::info!(
                document_number = &p.document_number,
                document_type_id = &p.document_type_id,
                "person document already exist"
            );

            return Err(CreateDomainFailure::Conflict(format!(
                "person with document number {} already exist",
                p.document_number
            )));
        }

        let person_id = self.person_repo.save(&person_ent).await?;

        Ok(person_id)
    }
}

#[async_trait]
impl FindAllPeopleUseCase for PersonService {
    async fn find_all_people(
        &self,
        query: &PaginationQuery,
    ) -> Result<Page<PersonPageSelector>, FindManyFailure> {
        Ok(self.person_repo.find_all(query).await?)
    }
}

#[async_trait]
impl UpdatePersonUseCase for PersonService {
    async fn update_person(
        &self,
        command: &UpdatePersonCommand,
    ) -> Result<(), UpdateDomainFailure> {
        tracing::info!("updating person {:?}", command);

        let person_ent = PersonEnt::try_from(command)?;

        let person = self
            .person_repo
            .find_by_id(person_ent.person_id.unwrap())
            .await?;
        if person.is_none() {
            tracing::info!(
                person_id = &person_ent.person_id.unwrap(),
                "person does not exist"
            );

            Err(UpdateDomainFailure::InvalidField(InvalidField::new(
                "person_id".into(),
                "person does not exist".into(),
            )))?;
        }

        let document_type = self
            .person_document_type_repo
            .find_by_id(person_ent.document_type_id)
            .await?;

        if document_type.is_none() {
            tracing::info!(
                document_type_id = &person_ent.document_type_id,
                "document type does not exist"
            );

            Err(UpdateDomainFailure::InvalidField(InvalidField::new(
                "document_type_id".into(),
                "document type does not exist".into(),
            )))?;
        }

        let person_gender = self
            .person_gender_repo
            .find_by_id(person_ent.gender_id)
            .await?;
        if person_gender.is_none() {
            tracing::info!(gender_id = &person_ent.gender_id, "gender does not exist");

            Err(UpdateDomainFailure::InvalidField(InvalidField::new(
                "gender_id".into(),
                "gender does not exist".into(),
            )))?;
        }

        let person_selector = self
            .person_repo
            .find_by_signature(&person_ent.signature)
            .await?;

        if let Some(c) = person_selector {
            tracing::info!(signature = &c.signature, "person signature already exist");

            return Err(UpdateDomainFailure::Conflict(
                "person with signature already exist".to_string(),
            ));
        }

        let someone_with_same_document = self
            .person_repo
            .find_another_by_document_and_type(
                person_ent.person_id.unwrap(),
                &person_ent.document_number,
                person_ent.document_type_id,
            )
            .await?;

        if let Some(someone_else) = someone_with_same_document {
            tracing::info!(
                document_number = &someone_else.document_number,
                document_type_id = &someone_else.document_type_id,
                "person document already exist"
            );

            Err(UpdateDomainFailure::Conflict(
                "person document already exist".to_string(),
            ))?;
        }

        self.person_repo.update(&person_ent).await?;

        Ok(())
    }
}

#[async_trait]
impl FindPersonDetailsUseCase for PersonService {
    async fn find_person_details(
        &self,
        person_id: i32,
    ) -> Result<PersonDetailsSelector, FindOneFailure> {
        let person_selector = self.person_repo.find_details_by_id(person_id).await?;

        if person_selector.is_none() {
            tracing::info!(person_id = &person_id, "person does not exist");

            Err(FindOneFailure::NotFound(
                "person does not exist".to_string(),
            ))?;
        }

        Ok(person_selector.unwrap())
    }
}
