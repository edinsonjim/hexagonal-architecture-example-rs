use crate::domain::selectors::product::ProductSelector;
use portal_schema::product;

impl From<product::Model> for ProductSelector {
    fn from(value: product::Model) -> Self {
        Self {
            product_id: value.product_id,
            signature: value.signature.to_owned(),
        }
    }
}
