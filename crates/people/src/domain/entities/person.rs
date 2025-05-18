use ids_std_domain::{
    api::failure::{CreateDomainFailure, UpdateDomainFailure},
    validation,
};
use ids_std_utils::{capitalizer, signer};

use crate::domain::commands::person::{CreatePersonCommand, UpdatePersonCommand};

#[derive(Debug, PartialEq)]
pub struct PersonEnt {
    pub person_id: Option<i32>,
    pub first_name: String,
    pub last_name: String,
    pub document_number: String,
    pub document_type_id: i32,
    pub gender_id: i32,
    pub signature: String,
}

impl PersonEnt {
    fn sign(
        person_id: Option<i32>,
        first_name: &str,
        last_name: &str,
        document_number: &str,
        document_type_id: i32,
        gender_id: i32,
    ) -> Self {
        let fingerprint = format!(
            "{}.{}.{}.{}.{}",
            first_name.trim(),
            last_name.trim(),
            gender_id,
            document_number.trim(),
            document_type_id
        )
        .to_lowercase();

        Self {
            person_id,
            first_name: capitalizer::capitalize(first_name),
            last_name: capitalizer::capitalize(last_name),
            document_number: document_number.trim().to_string(),
            document_type_id,
            gender_id,
            signature: signer::sign(fingerprint),
        }
    }
}

impl TryFrom<&CreatePersonCommand> for PersonEnt {
    type Error = CreateDomainFailure;

    fn try_from(value: &CreatePersonCommand) -> Result<Self, Self::Error> {
        validation::Validator::try_validate(value)?;

        Ok(Self::sign(
            None,
            &value.first_name,
            &value.last_name,
            &value.document_number,
            value.document_type_id,
            value.gender_id,
        ))
    }
}

impl TryFrom<&UpdatePersonCommand> for PersonEnt {
    type Error = UpdateDomainFailure;

    fn try_from(value: &UpdatePersonCommand) -> Result<Self, Self::Error> {
        validation::Validator::try_validate(value)?;

        Ok(Self::sign(
            Some(value.person_id),
            &value.first_name,
            &value.last_name,
            &value.document_number,
            value.document_type_id,
            value.gender_id,
        ))
    }
}

#[cfg(test)]
mod tests {

    use crate::domain::commands::person::{CreatePersonCommand, UpdatePersonCommand};

    use super::PersonEnt;

    #[test]
    fn it_create_a_valid_person() {
        let expected = PersonEnt {
            person_id: None,
            first_name: "Idesoft".to_string(),
            last_name: "Systems".to_string(),
            document_number: "ID3SOFT".to_string(),
            document_type_id: 1,
            gender_id: 1,
            signature: "87ca18b2b4e8fa897cc0bfdf04c58417".to_string(),
        };

        let command = CreatePersonCommand {
            first_name: " iDesoft ".to_string(),
            last_name: " sYstEms ".to_string(),
            document_number: " ID3SOFT ".to_string(),
            document_type_id: 1,
            gender_id: 1,
        };

        let result = PersonEnt::try_from(&command).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn it_create_a_valid_person_to_update() {
        let expected = PersonEnt {
            person_id: Some(1),
            first_name: "Idesoft".to_string(),
            last_name: "Systems".to_string(),
            document_number: "ID3SOFT".to_string(),
            document_type_id: 1,
            gender_id: 1,
            signature: "87ca18b2b4e8fa897cc0bfdf04c58417".to_string(),
        };

        let command = UpdatePersonCommand {
            first_name: " iDeSoft ".to_string(),
            last_name: " sYstEms ".to_string(),
            document_number: " ID3SOFT ".to_string(),
            document_type_id: 1,
            person_id: 1,
            gender_id: 1,
        };

        let result = PersonEnt::try_from(&command).unwrap();
        assert_eq!(result, expected)
    }
}
