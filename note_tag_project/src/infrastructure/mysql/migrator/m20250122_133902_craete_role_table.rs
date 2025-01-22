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
                    .table(Role::Table)
                        .if_not_exists()
                    .col(pk_auto(Role::Id).integer())
                    .col(string(Role::RoleName).not_null().unique_key())
                    .col(date_time(Role::CreateAt).not_null().default(Expr::current_timestamp()))
                    .col(date_time(Role::UpdatedAt).not_null().default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        

        manager
            .drop_table(Table::drop().table(Role::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Role {
    Table,
    Id,
    RoleName,
    CreateAt,
    UpdatedAt
    
}
