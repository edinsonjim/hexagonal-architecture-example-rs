use ids_std_utils::{
    capitalizer::{self},
    signer,
};

pub struct PersonGenderSignature {
    signature: String,
}

impl PersonGenderSignature {
    pub fn new(name: &str) -> Self {
        Self {
            signature: Self::sign(name),
        }
    }

    pub fn get(self) -> String {
        self.signature
    }

    fn sign(name: &str) -> String {
        let fingerprint = format!("{}", name).to_lowercase();

        signer::sign(fingerprint)
    }
}

pub struct PersonGenderName {
    name: String,
}

impl PersonGenderName {
    pub fn new(name: &str) -> Self {
        Self {
            name: capitalizer::capitalize(name),
        }
    }

    pub fn get(self) -> String {
        self.name
    }
}

#[cfg(test)]
mod tests {
    use super::PersonGenderName;

    #[test]
    fn it_create_a_valid_person_gender() {
        let expected = "Female";

        let result = PersonGenderName::new(" female ").get();

        assert_eq!(expected, result);
    }
}
