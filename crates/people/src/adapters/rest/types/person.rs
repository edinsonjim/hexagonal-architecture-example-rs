use serde::{Deserialize, Serialize};

use crate::domain::{
    selectors::person::{PersonDetailsSelector, PersonPageSelector},
    valuables::person::PersonFullName,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePersonParams {
    pub first_name: String,
    pub last_name: String,
    pub document_number: String,
    pub document_type_id: i32,
    pub gender_id: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePersonParams {
    pub first_name: String,
    pub last_name: String,
    pub document_number: String,
    pub document_type_id: i32,
    pub gender_id: i32,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonPageQuery {
    pub person_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub full_name: String,
    pub document_number: String,
    pub document_type_id: i32,
    pub document_type_name: String,
    pub gender_id: i32,
    pub gender_name: String,
}

impl From<&PersonPageSelector> for PersonPageQuery {
    fn from(value: &PersonPageSelector) -> Self {
        Self {
            person_id: value.person_id,
            first_name: value.first_name.to_string(),
            last_name: value.last_name.to_string(),
            full_name: value.full_name(),
            document_number: value.document_number.to_string(),
            document_type_id: value.document_type_id,
            document_type_name: value.document_type_name.to_string(),
            gender_id: value.gender_id,
            gender_name: value.gender_name.to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonDetailsQuery {
    pub person_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub full_name: String,
    pub document_number: String,
    pub document_type_id: i32,
    pub document_type_name: String,
    pub gender_id: i32,
    pub gender_name: String,
}

impl From<&PersonDetailsSelector> for PersonDetailsQuery {
    fn from(value: &PersonDetailsSelector) -> Self {
        PersonDetailsQuery {
            person_id: value.person_id,
            first_name: value.first_name.to_string(),
            last_name: value.last_name.to_string(),
            full_name: value.full_name(),
            document_number: value.document_number.to_string(),
            document_type_id: value.document_type_id,
            document_type_name: value.document_type_name.to_string(),
            gender_id: value.gender_id,
            gender_name: value.gender_name.to_string(),
        }
    }
}
