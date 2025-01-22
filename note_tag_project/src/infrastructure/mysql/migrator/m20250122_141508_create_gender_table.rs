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
                    .table(Gender::Table)
                    .if_not_exists()
                    .col(pk_auto(Gender::Id).integer())
                    .col(string(Gender::Detail).not_null().unique_key())
                    .col(date_time(Gender::CreateAt).not_null().default(Expr::current_timestamp()))
                    .col(date_time(Gender::UpdatedAt).not_null().default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        

        manager
            .drop_table(Table::drop().table(Gender::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Gender {
    Table,
    Id,
    Detail,
    CreateAt,
    UpdatedAt
}
