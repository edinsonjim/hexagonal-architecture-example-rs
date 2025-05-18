use ids_std_utils::signer;

pub struct ProductSignature {
    signature: String,
}

impl ProductSignature {
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
