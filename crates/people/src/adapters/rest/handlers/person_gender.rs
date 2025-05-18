use crate::{
    adapters::rest::types::person_gender::{CreatePersonGenderParams, UpdatePersonGenderParams},
    domain::{
        commands::person_gender::{CreatePersonGenderCommand, UpdatePersonGenderCommand},
        ports::api::person_gender::{CreatePersonGenderUseCase, UpdatePersonGenderUseCase},
        services::person_gender::PersonGenderService,
    },
};
use ids_std_rest_api::{
    failure::ApiFailure,
    replier::Replier,
    types::{created::Created, result::ApiResult},
};
use lumx_axum::axum::{extract::Path, http::StatusCode, Json};
use lumx_axum::extractor::Component;

pub async fn create_person_gender(
    Component(uc): Component<PersonGenderService>,
    Json(payload): Json<CreatePersonGenderParams>,
) -> ApiResult<Created<i32>> {
    tracing::info!("creating person gender {:?}", payload);

    let create_person_gender_cmd = CreatePersonGenderCommand {
        name: payload.name,
        summary: payload.summary,
    };

    uc.create(&create_person_gender_cmd)
        .await
        .map(|id| Replier::ok(Created::new(id)))
        .map_err(ApiFailure::from)
}

pub async fn update_person_gender(
    Path(person_gender_id): Path<i32>,
    Component(uc): Component<PersonGenderService>,
    Json(payload): Json<UpdatePersonGenderParams>,
) -> Result<StatusCode, ApiFailure> {
    tracing::info!("updating person gender {:?}", payload);

    let command = UpdatePersonGenderCommand {
        person_gender_id,
        name: payload.name,
        summary: payload.summary,
    };

    uc.update_person_gender(&command)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(ApiFailure::from)
}
