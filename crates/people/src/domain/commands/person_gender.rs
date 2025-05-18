use validator::Validate;

#[derive(Validate, Debug, Clone)]
pub struct CreatePersonGenderCommand {
    #[validate(length(min = 1, max = 100))]
    pub name: String,

    pub summary: Option<String>,
}

#[derive(Debug, Validate, Clone)]
pub struct UpdatePersonGenderCommand {
    pub person_gender_id: i32,

    #[validate(length(min = 1, max = 100))]
    pub name: String,

    pub summary: Option<String>,
}
