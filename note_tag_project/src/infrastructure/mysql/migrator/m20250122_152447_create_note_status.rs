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
                    .table(NoteStatus::Table)
                    .if_not_exists()
                    .col(pk_auto(NoteStatus::Id).integer())
                    .col(string(NoteStatus::StatusDetail))
                    .col(
                        ColumnDef::new(NoteStatus::CreatedAt)
                            .timestamp()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
                    )
                    .col(
                        ColumnDef::new(NoteStatus::UpdatedAt)
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
            .drop_table(Table::drop().table(NoteStatus::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum NoteStatus {
    Table,
    Id,
    StatusDetail,
    CreatedAt,
    UpdatedAt
}
