use crate::m20241025_093336_create_product_family_table::ProductFamily;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Product::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Product::ProductId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Product::FamilyId).integer().not_null())
                    .col(ColumnDef::new(Product::Name).string_len(100).not_null())
                    .col(ColumnDef::new(Product::Summary).string().null())
                    .col(ColumnDef::new(Product::Purchasable).boolean().not_null())
                    .col(ColumnDef::new(Product::Saleable).boolean().not_null())
                    .col(
                        ColumnDef::new(Product::SaleableWithoutStock)
                            .boolean()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Product::Signature).string_len(50).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Product::Table, Product::FamilyId)
                            .to(ProductFamily::Table, ProductFamily::ProductFamilyId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Product::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Product {
    Table,
    ProductId,
    FamilyId,
    Name,
    Summary,
    Purchasable,
    Saleable,
    SaleableWithoutStock,
    Signature,
}
