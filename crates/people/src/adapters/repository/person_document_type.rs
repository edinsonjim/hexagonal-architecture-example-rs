use std::sync::Arc;

use crate::domain::{
    ports::spi::person_document_type::PersonDocumentTypeRepository,
    queries::person_document_type::PersonDocumentTypeQuery,
    selectors::person_document_type::{PersonDocumentTypePageSelector, PersonDocumentTypeSelector},
};
use async_trait::async_trait;
use ids_std_domain::{
    pagination::{Page, PaginationQuery},
    spi::failure::SelectRepoFailure,
};
use ids_std_sea::convert::into::IntoDomain;
use ids_std_sea::paginator;
use lumx_sea_orm::sea_orm::PaginatorTrait;
use lumx_sea_orm::sea_orm::QueryFilter;
use lumx_sea_orm::sea_orm::QueryOrder;
use lumx_sea_orm::sea_orm::{ColumnTrait, DbConn, EntityTrait};
use portal_schema::{self, person_document_type};

#[derive(Clone)]
pub struct PersonDocumentTypeSeaRepository {
    db: Arc<DbConn>,
}

impl PersonDocumentTypeSeaRepository {
    pub fn new(db: &Arc<DbConn>) -> Self {
        Self {
            db: Arc::clone(&db),
        }
    }
}

#[async_trait]
impl PersonDocumentTypeRepository for PersonDocumentTypeSeaRepository {
    async fn find_by_id(
        &self,
        id: i32,
    ) -> Result<Option<PersonDocumentTypeSelector>, SelectRepoFailure> {
        let maybe_model = person_document_type::Entity::find_by_id(id)
            .one(self.db.as_ref())
            .await
            .map_err(|err| err.into_domain())?
            .and_then(|model| {
                Some(PersonDocumentTypeSelector {
                    person_document_type_id: model.person_document_type_id,
                    name: model.name,
                    summary: model.summary,
                    signature: model.signature,
                })
            });

        Ok(maybe_model)
    }

    async fn find_all(
        &self,
        query: &PaginationQuery,
    ) -> Result<Page<PersonDocumentTypePageSelector>, SelectRepoFailure> {
        let paginator = person_document_type::Entity::find()
            .order_by_desc(person_document_type::Column::PersonDocumentTypeId)
            .paginate(self.db.as_ref(), query.page_size);

        paginator::fetch_page(&paginator, query, |model| PersonDocumentTypePageSelector {
            person_document_type_id: model.person_document_type_id,
            name: model.name.to_string(),
            summary: model.summary.clone(),
            signature: model.signature.to_string(),
        })
        .await
    }

    async fn find_by_criteria(
        &self,
        query: &PaginationQuery,
        criteria: &PersonDocumentTypeQuery,
    ) -> Result<Page<PersonDocumentTypePageSelector>, SelectRepoFailure> {
        let paginator = person_document_type::Entity::find()
            .filter(person_document_type::Column::Name.contains(criteria.name.to_string()))
            .order_by_desc(person_document_type::Column::PersonDocumentTypeId)
            .paginate(self.db.as_ref(), query.page_size);

        paginator::fetch_page(&paginator, query, |model| PersonDocumentTypePageSelector {
            person_document_type_id: model.person_document_type_id,
            name: model.name.to_string(),
            summary: model.summary.clone(),
            signature: model.signature.to_string(),
        })
        .await
    }
}
