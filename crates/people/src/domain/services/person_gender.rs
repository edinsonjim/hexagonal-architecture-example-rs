use std::sync::Arc;

use async_trait::async_trait;
use ids_std_domain::{
    api::failure::{CreateDomainFailure, FindManyFailure, UpdateDomainFailure},
    pagination::{Page, PaginationQuery},
    validation,
};

use crate::domain::{
    changes::person_gender::{AddPersonGender, UpdatePersonGender},
    commands::person_gender::{CreatePersonGenderCommand, UpdatePersonGenderCommand},
    ports::{
        api::person_gender::{
            CreatePersonGenderUseCase, FindPersonGenderByCriteriaUseCase, UpdatePersonGenderUseCase,
        },
        spi::person_gender::PersonGenderRepository,
    },
    queries::person_gender::PersonGenderQuery,
    selectors::person_gender::PersonGenderPageSelector,
    valuables::person_gender::{PersonGenderName, PersonGenderSignature},
};

#[derive(Clone)]
pub struct PersonGenderService {
    person_gender_repo: Arc<dyn PersonGenderRepository>,
}

impl PersonGenderService {
    pub fn new(person_gender_repo: Arc<dyn PersonGenderRepository>) -> Self {
        Self { person_gender_repo }
    }
}

#[async_trait]
impl FindPersonGenderByCriteriaUseCase for PersonGenderService {
    async fn find_person_gender_by_criteria(
        &self,
        query: &PaginationQuery,
        criteria: &PersonGenderQuery,
    ) -> Result<Page<PersonGenderPageSelector>, FindManyFailure> {
        Ok(self
            .person_gender_repo
            .find_by_criteria(query, criteria)
            .await?)
    }
}

#[async_trait]
impl CreatePersonGenderUseCase for PersonGenderService {
    async fn create(&self, cmd: &CreatePersonGenderCommand) -> Result<i32, CreateDomainFailure> {
        tracing::info!("creating person gender {:?}", cmd);

        ids_std_domain::validation::Validator::try_validate(cmd)?;

        let gender_signature = PersonGenderSignature::new(cmd.name.as_str()).get();
        let gender_name = PersonGenderName::new(cmd.name.as_str()).get();

        let person_gender = self
            .person_gender_repo
            .find_by_signature(gender_signature.as_str())
            .await?;

        if let Some(selector) = person_gender {
            tracing::info!(
                signature = &selector.signature,
                "person gender signature already exist"
            );

            return Err(CreateDomainFailure::Conflict(
                "gender with signature already exist".to_string(),
            ));
        }

        let event = AddPersonGender {
            name: gender_name.to_owned(),
            summary: cmd.summary.to_owned(),
            signature: gender_signature.to_owned(),
        };

        let person_gender_id = self.person_gender_repo.save(event).await?;

        Ok(person_gender_id)
    }
}

#[async_trait]
impl UpdatePersonGenderUseCase for PersonGenderService {
    async fn update_person_gender(
        &self,
        command: &UpdatePersonGenderCommand,
    ) -> Result<(), UpdateDomainFailure> {
        tracing::info!("updating person gender {:?}", command);

        validation::Validator::try_validate(command)?;

        let gender_signature = PersonGenderSignature::new(command.name.as_str()).get();
        let gender_name = PersonGenderName::new(command.name.as_str()).get();

        let other_person_gender = self
            .person_gender_repo
            .find_other_by_signature(command.person_gender_id, &gender_signature)
            .await?;

        if let Some(selector) = other_person_gender {
            tracing::info!(
                signature = &selector.signature,
                "person gender signature already exist"
            );

            Err(UpdateDomainFailure::Conflict(
                "gender with signature already exist".to_string(),
            ))?;
        }

        let event = UpdatePersonGender {
            person_gender_id: command.person_gender_id,
            name: gender_name.to_owned(),
            summary: command.summary.to_owned(),
            signature: gender_signature.to_owned(),
        };
        self.person_gender_repo.update(&event).await?;

        Ok(())
    }
}
