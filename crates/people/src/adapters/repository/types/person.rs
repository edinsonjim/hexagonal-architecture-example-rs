use lumx_sea_orm::sea_orm;
use portal_schema::person;

use crate::domain::selectors::person::{PersonDetailsSelector, PersonPageSelector, PersonSelector};

#[derive(sea_orm::FromQueryResult)]
pub struct PersonAndGenderAndDocument {
    pub person_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub document_number: String,
    pub document_type_id: i32,
    pub document_type_name: String,
    pub gender_id: i32,
    pub gender_name: String,
    pub signature: String,
}

impl From<PersonAndGenderAndDocument> for PersonDetailsSelector {
    fn from(value: PersonAndGenderAndDocument) -> Self {
        Self {
            person_id: value.person_id,
            first_name: value.first_name,
            last_name: value.last_name,
            document_number: value.document_number,
            document_type_id: value.document_type_id,
            document_type_name: value.document_type_name,
            gender_id: value.gender_id,
            gender_name: value.gender_name,
            signature: value.signature,
        }
    }
}

impl From<person::Model> for PersonSelector {
    fn from(value: person::Model) -> Self {
        Self {
            person_id: value.person_id,
            first_name: value.first_name.to_string(),
            last_name: value.last_name.to_string(),
            document_number: value.document_number.to_string(),
            document_type_id: value.document_type_id,
            gender_id: value.gender_id,
            signature: value.signature.to_string(),
        }
    }
}

impl From<&PersonAndGenderAndDocument> for PersonPageSelector {
    fn from(value: &PersonAndGenderAndDocument) -> Self {
        Self {
            person_id: value.person_id,
            first_name: value.first_name.to_string(),
            last_name: value.last_name.to_string(),
            document_number: value.document_number.to_string(),
            document_type_id: value.document_type_id,
            document_type_name: value.document_type_name.to_string(),
            gender_id: value.gender_id,
            gender_name: value.gender_name.to_string(),
            signature: value.signature.to_string(),
        }
    }
}
