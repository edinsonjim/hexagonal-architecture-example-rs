use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProductParams {
    pub name: String,

    pub summary: Option<String>,

    pub family_id: i32,

    pub purchasable: bool,

    pub saleable: bool,

    pub saleable_without_stock: bool,
}
