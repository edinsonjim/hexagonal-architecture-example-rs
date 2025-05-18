use crate::adapters::rest::types::person::{
    CreatePersonParams, PersonDetailsQuery, PersonPageQuery, UpdatePersonParams,
};
use crate::domain::commands::person::{CreatePersonCommand, UpdatePersonCommand};
use crate::domain::ports::api::person::{
    CreatePersonUseCase, FindAllPeopleUseCase, FindPersonDetailsUseCase, UpdatePersonUseCase,
};
use crate::domain::services::person::PersonService;
use std::sync::Arc;

use ids_std_rest_api::types::pagination::{Paged, PaginationParams};
use ids_std_rest_api::{
    failure::ApiFailure,
    replier::Replier,
    types::{created::Created, result::ApiResult},
};
use lumx_axum::axum::extract::{Path, Query};
use lumx_axum::axum::http::StatusCode;
use lumx_axum::axum::{Extension, Json};
use lumx_axum::extractor::Component;
use passport_core::user::ClaimsPrincipal;

pub async fn create_person(
    Component(uc): Component<PersonService>,
    Extension(principal): Extension<Arc<dyn ClaimsPrincipal>>,
    Json(payload): Json<CreatePersonParams>,
) -> ApiResult<Created<i32>> {
    tracing::info!(?payload, creator_id = principal.sub_id(), "creating person");

    let create_person_cmd = CreatePersonCommand {
        first_name: payload.first_name,
        last_name: payload.last_name,
        document_number: payload.document_number,
        document_type_id: payload.document_type_id,
        gender_id: payload.gender_id,
    };

    uc.create(&create_person_cmd)
        .await
        .map(|id| Replier::ok(Created::new(id)))
        .map_err(ApiFailure::from)
}

pub async fn find_all_people(
    Component(uc): Component<PersonService>,
    Query(payload): Query<PaginationParams>,
) -> ApiResult<Paged<PersonPageQuery>> {
    tracing::info!("find and paginate all people {:?}", payload);

    let people = uc.find_all_people(&payload.into()).await?;

    Ok(Replier::ok(Paged::from(&people, |item| {
        PersonPageQuery::from(item)
    })))
}

pub async fn update_person(
    Path(person_id): Path<i32>,
    Component(uc): Component<PersonService>,
    Json(payload): Json<UpdatePersonParams>,
) -> Result<StatusCode, ApiFailure> {
    tracing::info!("updating person {:?}", payload);

    let update_person_cmd = UpdatePersonCommand {
        first_name: payload.first_name,
        last_name: payload.last_name,
        document_number: payload.document_number,
        document_type_id: payload.document_type_id,
        person_id,
        gender_id: payload.gender_id,
    };
    uc.update_person(&update_person_cmd)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(ApiFailure::from)
}

pub async fn find_person(
    Path(person_id): Path<i32>,
    Component(uc): Component<PersonService>,
) -> ApiResult<PersonDetailsQuery> {
    tracing::info!(person_id = person_id, "finding person");

    uc.find_person_details(person_id)
        .await
        .map(|person| Replier::ok(PersonDetailsQuery::from(&person)))
        .map_err(ApiFailure::from)
}
