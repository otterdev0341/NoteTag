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
                    .col(
                        ColumnDef::new(Gender::CreatedAt)
                            .timestamp()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
                    )
                    .col(
                        ColumnDef::new(Gender::UpdatedAt)
                            .timestamp()
                            .extra("DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP".to_owned()),
                    )
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
    CreatedAt,
    UpdatedAt
}
