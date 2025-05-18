use validator::Validate;

#[derive(Validate, Debug, Clone)]
pub struct CreatePersonCommand {
    #[validate(length(min = 1, max = 100))]
    pub first_name: String,

    #[validate(length(min = 1, max = 100))]
    pub last_name: String,

    #[validate(length(min = 1, max = 100))]
    pub document_number: String,

    pub document_type_id: i32,

    pub gender_id: i32,
}

#[derive(Debug, Validate, Clone)]
pub struct UpdatePersonCommand {
    pub person_id: i32,

    #[validate(length(min = 1, max = 100))]
    pub first_name: String,

    #[validate(length(min = 1, max = 100))]
    pub last_name: String,

    #[validate(length(min = 1, max = 100))]
    pub document_number: String,

    pub document_type_id: i32,

    pub gender_id: i32,
}
