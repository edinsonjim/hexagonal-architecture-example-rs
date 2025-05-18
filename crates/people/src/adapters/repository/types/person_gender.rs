use portal_schema::person_gender;

use crate::domain::selectors::person_gender::PersonGenderSelector;

impl From<person_gender::Model> for PersonGenderSelector {
    fn from(value: person_gender::Model) -> Self {
        Self {
            person_gender_id: value.person_gender_id,
            name: value.name.to_string(),
            summary: value.summary,
            signature: value.signature.to_string(),
        }
    }
}
