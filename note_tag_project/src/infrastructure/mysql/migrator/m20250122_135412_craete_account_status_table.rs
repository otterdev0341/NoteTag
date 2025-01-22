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
                    .table(AccountStatus::Table)
                    .if_not_exists()
                    .col(pk_auto(AccountStatus::Id).integer())
                    .col(string(AccountStatus::StatusDetail).not_null().unique_key())
                    .col(date_time(AccountStatus::CreateAt).not_null().default(Expr::current_timestamp()))
                    .col(date_time(AccountStatus::UpdatedAt).not_null().default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        

        manager
            .drop_table(Table::drop().table(AccountStatus::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum AccountStatus {
    Table,
    Id,
    StatusDetail,
    CreateAt,
    UpdatedAt
    
}
