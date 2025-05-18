pub use sea_orm_migration::prelude::*;

mod m20200118_120326_create_user_table;
mod m20240207_120014_create_person_gender_table;
mod m20240207_211848_create_person_document_type_table;
mod m20240207_211901_create_person_table;
mod m20241025_093336_create_product_family_table;
mod m20241025_094129_create_product_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240207_120014_create_person_gender_table::Migration),
            Box::new(m20240207_211848_create_person_document_type_table::Migration),
            Box::new(m20240207_211901_create_person_table::Migration),
            Box::new(m20241025_093336_create_product_family_table::Migration),
            Box::new(m20241025_094129_create_product_table::Migration),
            Box::new(m20200118_120326_create_user_table::Migration),
        ]
    }
}
