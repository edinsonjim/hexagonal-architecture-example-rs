use sea_orm_migration::prelude::*;

use crate::{
    m20240207_120014_create_person_gender_table::PersonGender,
    m20240207_211848_create_person_document_type_table::PersonDocumentType,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Person::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Person::PersonId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Person::FirstName).string_len(100).not_null())
                    .col(ColumnDef::new(Person::LastName).string_len(100).not_null())
                    .col(
                        ColumnDef::new(Person::DocumentNumber)
                            .string_len(50)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Person::DocumentTypeId).integer().not_null())
                    .col(ColumnDef::new(Person::GenderId).integer().not_null())
                    .col(ColumnDef::new(Person::Signature).string_len(50).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Person::Table, Person::DocumentTypeId)
                            .to(
                                PersonDocumentType::Table,
                                PersonDocumentType::PersonDocumentTypeId,
                            ),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Person::Table, Person::GenderId)
                            .to(PersonGender::Table, PersonGender::PersonGenderId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Person::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Person {
    Table,
    PersonId,
    FirstName,
    LastName,
    DocumentNumber,
    DocumentTypeId,
    GenderId,
    Signature,
}
