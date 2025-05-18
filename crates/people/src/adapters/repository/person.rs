use std::sync::Arc;

use async_trait::async_trait;
use ids_std_domain::pagination::{Page, PaginationQuery};
use ids_std_domain::spi::failure::{SaveRepoFailure, SelectRepoFailure};
use ids_std_sea::convert::into::IntoDomain;
use ids_std_sea::paginator;
use lumx_sea_orm::sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DbConn, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect, RelationTrait,
};
use portal_schema::{person, person_document_type, person_gender};

use crate::domain::entities::person::PersonEnt;
use crate::domain::ports::spi::person::PersonRepository;
use crate::domain::selectors::person::{PersonDetailsSelector, PersonPageSelector, PersonSelector};

use super::types::person::PersonAndGenderAndDocument;

#[derive(Clone)]
pub struct PersonSeaRepository {
    db: Arc<DbConn>,
}

impl PersonSeaRepository {
    pub fn new(db: &Arc<DbConn>) -> Self {
        Self {
            db: Arc::clone(&db),
        }
    }
}

#[async_trait]
impl PersonRepository for PersonSeaRepository {
    async fn save(&self, person: &PersonEnt) -> Result<i32, SaveRepoFailure> {
        let person_model = person::ActiveModel {
            first_name: ActiveValue::Set(person.first_name.to_owned()),
            last_name: ActiveValue::Set(person.last_name.to_owned()),
            document_number: ActiveValue::Set(person.document_number.to_owned()),
            document_type_id: ActiveValue::Set(person.document_type_id),
            gender_id: ActiveValue::Set(person.gender_id),
            signature: ActiveValue::Set(person.signature.to_owned()),
            ..Default::default()
        };

        person_model
            .save(self.db.as_ref())
            .await
            .map(|model| model.person_id.unwrap())
            .map_err(|err| err.into_domain())
    }

    async fn update(&self, person: &PersonEnt) -> Result<(), SaveRepoFailure> {
        let result = person::Entity::find_by_id(person.person_id.unwrap())
            .one(self.db.as_ref())
            .await
            .map_err(|err| err.into_domain())?;

        let mut person_to_modified: person::ActiveModel = result.unwrap().into();
        person_to_modified.first_name = ActiveValue::Set(person.first_name.to_owned());
        person_to_modified.last_name = ActiveValue::Set(person.last_name.to_owned());
        person_to_modified.document_number = ActiveValue::Set(person.document_number.to_owned());
        person_to_modified.document_type_id = ActiveValue::Set(person.document_type_id);
        person_to_modified.gender_id = ActiveValue::Set(person.gender_id);
        person_to_modified.signature = ActiveValue::Set(person.signature.to_owned());

        person_to_modified
            .save(self.db.as_ref())
            .await
            .map(|_| {})
            .map_err(|err| err.into_domain())
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<PersonSelector>, SelectRepoFailure> {
        let maybe_model = person::Entity::find_by_id(id)
            .one(self.db.as_ref())
            .await
            .map_err(|err| err.into_domain())?
            .map(PersonSelector::from);

        Ok(maybe_model)
    }

    async fn find_details_by_id(
        &self,
        id: i32,
    ) -> Result<Option<PersonDetailsSelector>, SelectRepoFailure> {
        let maybe_model = person::Entity::find_by_id(id)
            .column_as(person_gender::Column::Name, "gender_name")
            .column_as(person_document_type::Column::Name, "document_type_name")
            .join(
                lumx_sea_orm::sea_orm::JoinType::LeftJoin,
                person::Relation::PersonDocumentType.def(),
            )
            .join(
                lumx_sea_orm::sea_orm::JoinType::LeftJoin,
                person::Relation::PersonGender.def(),
            )
            .into_model::<PersonAndGenderAndDocument>()
            .one(self.db.as_ref())
            .await
            .map_err(|err| err.into_domain())?
            .map(|model| PersonDetailsSelector::from(model));

        Ok(maybe_model)
    }

    async fn find_by_signature(
        &self,
        signature: &str,
    ) -> Result<Option<PersonSelector>, SelectRepoFailure> {
        let maybe_model = person::Entity::find()
            .filter(person::Column::Signature.eq(signature))
            .one(self.db.as_ref())
            .await
            .map_err(|err| err.into_domain())?
            .map(PersonSelector::from);

        Ok(maybe_model)
    }

    async fn find_by_document_and_type(
        &self,
        document_number: &str,
        document_type_id: i32,
    ) -> Result<Option<PersonSelector>, SelectRepoFailure> {
        let maybe_model = person::Entity::find()
            .filter(
                person::Column::DocumentNumber
                    .eq(document_number)
                    .and(person::Column::DocumentTypeId.eq(document_type_id)),
            )
            .one(self.db.as_ref())
            .await
            .map_err(|err| err.into_domain())?
            .map(PersonSelector::from);

        Ok(maybe_model)
    }

    async fn find_another_by_document_and_type(
        &self,
        self_id: i32,
        document_number: &str,
        document_type_id: i32,
    ) -> Result<Option<PersonSelector>, SelectRepoFailure> {
        let model_opt = person::Entity::find()
            .filter(
                person::Column::DocumentNumber
                    .eq(document_number)
                    .and(person::Column::DocumentTypeId.eq(document_type_id))
                    .and(person::Column::PersonId.is_not_in(vec![self_id])),
            )
            .one(self.db.as_ref())
            .await
            .map_err(|err| err.into_domain())?;

        Ok(model_opt.map(PersonSelector::from))
    }

    async fn find_all(
        &self,
        query: &PaginationQuery,
    ) -> Result<Page<PersonPageSelector>, SelectRepoFailure> {
        let paginator = person::Entity::find()
            .column_as(person_gender::Column::Name, "gender_name")
            .column_as(person_document_type::Column::Name, "document_type_name")
            .join(
                lumx_sea_orm::sea_orm::JoinType::LeftJoin,
                person::Relation::PersonDocumentType.def(),
            )
            .join(
                lumx_sea_orm::sea_orm::JoinType::LeftJoin,
                person::Relation::PersonGender.def(),
            )
            .order_by_desc(person::Column::PersonId)
            .into_model::<PersonAndGenderAndDocument>()
            .paginate(self.db.as_ref(), query.page_size);

        paginator::fetch_page(&paginator, query, |model| PersonPageSelector::from(model)).await
    }
}
