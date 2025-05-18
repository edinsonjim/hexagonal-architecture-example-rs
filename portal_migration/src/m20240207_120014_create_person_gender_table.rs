use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PersonGender::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PersonGender::PersonGenderId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PersonGender::Name)
                            .string_len(100)
                            .not_null(),
                    )
                    .col(ColumnDef::new(PersonGender::Summary).string().null())
                    .col(
                        ColumnDef::new(PersonGender::Signature)
                            .string_len(50)
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PersonGender::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum PersonGender {
    Table,
    PersonGenderId,
    Name,
    Summary,
    Signature,
}
