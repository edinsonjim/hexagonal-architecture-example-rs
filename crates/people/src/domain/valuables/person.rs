pub trait PersonFullName {
    fn full_name(&self) -> String;
}

pub struct FullName {
    pub first_name: String,
    pub last_name: String,
}

impl FullName {
    pub fn new(first_name: &String, last_name: &String) -> Self {
        Self {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
        }
    }

    pub fn get(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}
