use crate::domain::selectors::family::ProductFamilySelector;
use portal_schema::product_family;

impl From<product_family::Model> for ProductFamilySelector {
    fn from(model: product_family::Model) -> Self {
        Self {
            product_family_id: model.product_family_id,
        }
    }
}
