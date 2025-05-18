use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PersonDocumentType::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PersonDocumentType::PersonDocumentTypeId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PersonDocumentType::Name)
                            .string_len(100)
                            .not_null(),
                    )
                    .col(ColumnDef::new(PersonDocumentType::Summary).string().null())
                    .col(
                        ColumnDef::new(PersonDocumentType::Signature)
                            .string_len(100)
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PersonDocumentType::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum PersonDocumentType {
    Table,
    PersonDocumentTypeId,
    Name,
    Summary,
    Signature,
}
