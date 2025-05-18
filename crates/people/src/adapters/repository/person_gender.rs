use std::sync::Arc;

use async_trait::async_trait;
use ids_std_domain::{
    pagination::{Page, PaginationQuery},
    spi::failure::{SaveRepoFailure, SelectRepoFailure},
};
use ids_std_sea::convert::into::IntoDomain;
use ids_std_sea::paginator;
use lumx_sea_orm::sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DbConn, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder,
};
use portal_schema::person_gender;

use crate::domain::{
    changes::person_gender::{AddPersonGender, UpdatePersonGender},
    ports::spi::person_gender::PersonGenderRepository,
    queries::person_gender::PersonGenderQuery,
    selectors::person_gender::{PersonGenderPageSelector, PersonGenderSelector},
};

#[derive(Clone)]
pub struct PersonGenderSeaRepository {
    db: Arc<DbConn>,
}

impl PersonGenderSeaRepository {
    pub fn new(db: &Arc<DbConn>) -> Self {
        Self {
            db: Arc::clone(&db),
        }
    }
}

#[async_trait]
impl PersonGenderRepository for PersonGenderSeaRepository {
    async fn find_by_id(&self, id: i32) -> Result<Option<PersonGenderSelector>, SelectRepoFailure> {
        let maybe_model = person_gender::Entity::find_by_id(id)
            .one(self.db.as_ref())
            .await
            .map_err(|err| err.into_domain())?
            .and_then(|model| {
                Some(PersonGenderSelector {
                    person_gender_id: model.person_gender_id,
                    name: model.name,
                    summary: model.summary,
                    signature: model.signature,
                })
            });

        Ok(maybe_model)
    }

    async fn find_by_criteria(
        &self,
        query: &PaginationQuery,
        criteria: &PersonGenderQuery,
    ) -> Result<Page<PersonGenderPageSelector>, SelectRepoFailure> {
        let paginator = person_gender::Entity::find()
            .filter(person_gender::Column::Name.contains(criteria.name.to_string()))
            .order_by_desc(person_gender::Column::PersonGenderId)
            .paginate(self.db.as_ref(), query.page_size);

        paginator::fetch_page(&paginator, query, |model| PersonGenderPageSelector {
            person_gender_id: model.person_gender_id,
            name: model.name.to_string(),
            summary: model.summary.clone(),
            signature: model.signature.to_string(),
        })
        .await
    }

    async fn find_by_signature(
        &self,
        signature: &str,
    ) -> Result<Option<PersonGenderSelector>, SelectRepoFailure> {
        let model_opt = person_gender::Entity::find()
            .filter(person_gender::Column::Signature.eq(signature))
            .one(self.db.as_ref())
            .await
            .map_err(|err| err.into_domain())?
            .map(PersonGenderSelector::from);

        Ok(model_opt)
    }

    async fn find_other_by_signature(
        &self,
        self_id: i32,
        signature: &str,
    ) -> Result<Option<PersonGenderSelector>, SelectRepoFailure> {
        let model_opt = person_gender::Entity::find()
            .filter(
                person_gender::Column::Signature
                    .eq(signature)
                    .and(person_gender::Column::PersonGenderId.is_not_in(vec![self_id])),
            )
            .one(self.db.as_ref())
            .await
            .map_err(|err| err.into_domain())?
            .map(PersonGenderSelector::from);

        Ok(model_opt)
    }

    async fn save(&self, event: AddPersonGender) -> Result<i32, SaveRepoFailure> {
        let person_gender_model = person_gender::ActiveModel {
            name: ActiveValue::Set(event.name.to_owned()),
            summary: ActiveValue::Set(event.summary.to_owned()),
            signature: ActiveValue::Set(event.signature.to_owned()),
            ..Default::default()
        };

        person_gender_model
            .save(self.db.as_ref())
            .await
            .map(|model| model.person_gender_id.unwrap())
            .map_err(|err| err.into_domain())
    }

    async fn update(&self, event: &UpdatePersonGender) -> Result<(), SaveRepoFailure> {
        let model_opt = person_gender::Entity::find_by_id(event.person_gender_id)
            .one(self.db.as_ref())
            .await
            .map_err(|err| err.into_domain())?;

        let mut gender_to_modified: person_gender::ActiveModel = model_opt.unwrap().into();
        gender_to_modified.name = ActiveValue::Set(event.name.to_owned());
        gender_to_modified.summary = ActiveValue::Set(event.summary.to_owned());
        gender_to_modified.signature = ActiveValue::Set(event.signature.to_owned());

        gender_to_modified
            .save(self.db.as_ref())
            .await
            .map(|_| {})
            .map_err(|err| err.into_domain())
    }
}
