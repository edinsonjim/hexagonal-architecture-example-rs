#[derive(Debug, Clone)]
pub struct PersonGenderSelector {
    pub person_gender_id: i32,
    pub name: String,
    pub summary: Option<String>,
    pub signature: String,
}

#[derive(Debug, Clone)]
pub struct PersonGenderPageSelector {
    pub person_gender_id: i32,
    pub name: String,
    pub summary: Option<String>,
    pub signature: String,
}
