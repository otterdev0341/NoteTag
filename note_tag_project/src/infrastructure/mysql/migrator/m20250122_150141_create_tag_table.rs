use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        

        manager
            .create_table(
                Table::create()
                    .table(Tag::Table)
                        .if_not_exists()
                    .col(pk_auto(Tag::Id).integer())
                    .col(string(Tag::TagName).not_null().unique_key())
                    .col(date_time(Tag::CreateAt).not_null().default(Expr::current_timestamp()))
                    .col(date_time(Tag::UpdatedAt).not_null().default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        

        manager
            .drop_table(Table::drop().table(Tag::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Tag {
    Table,
    Id,
    TagName,
    CreateAt,
    UpdatedAt
}
