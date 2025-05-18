use crate::domain::valuables::person::{FullName, PersonFullName};

pub struct PersonSelector {
    pub person_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub document_number: String,
    pub document_type_id: i32,
    pub gender_id: i32,
    pub signature: String,
}

pub struct PersonDetailsSelector {
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

impl PersonFullName for PersonDetailsSelector {
    fn full_name(&self) -> String {
        FullName::new(&self.first_name, &self.last_name).get()
    }
}

pub struct PersonPageSelector {
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

impl PersonFullName for PersonPageSelector {
    fn full_name(&self) -> String {
        FullName::new(&self.first_name, &self.last_name).get()
    }
}
