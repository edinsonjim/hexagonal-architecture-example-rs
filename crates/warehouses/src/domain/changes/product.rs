pub struct AddProduct {
    pub name: String,
    pub summary: Option<String>,
    pub family_id: i32,
    pub purchasable: bool,
    pub saleable: bool,
    pub saleable_without_stock: bool,
    pub signature: String,
}
