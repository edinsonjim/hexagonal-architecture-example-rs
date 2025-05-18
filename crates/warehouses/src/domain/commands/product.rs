use validator::Validate;

#[derive(Validate, Debug, Clone)]
pub struct CreateProductCommand {
    #[validate(length(min = 1, max = 100))]
    pub name: String,

    pub summary: Option<String>,

    pub family_id: i32,

    pub purchasable: bool,

    pub saleable: bool,

    pub saleable_without_stock: bool,
}
