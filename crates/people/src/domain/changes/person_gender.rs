pub struct AddPersonGender {
    pub name: String,
    pub summary: Option<String>,
    pub signature: String,
}

pub struct UpdatePersonGender {
    pub person_gender_id: i32,
    pub name: String,
    pub summary: Option<String>,
    pub signature: String,
}
