use sea_orm_migration::{prelude::*, schema::*};

use super::{m20220101_000001_create_user_table::User, m20250122_151926_create_note_hex_color::NoteHexColor, m20250122_152447_create_note_status::NoteStatus};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        

        manager
            .create_table(
                Table::create()
                    .table(Note::Table)
                        .if_not_exists()
                    .col(pk_auto(Note::Id).integer())
                    .col(string_len(Note::Title,255).not_null())
                    .col(string(Note::Detail).not_null())
                    .col(integer(Note::UserId).not_null())
                        .foreign_key(ForeignKey::create()
                        .name("fk_note_user")
                        .from(Note::Table, Note::UserId)
                        .to(User::Table, User::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade)
                    )
                    // fk
                    .col(integer(Note::Color).not_null())
                        .foreign_key(ForeignKey::create()
                        .name("fk_note_color")
                        .from(Note::Table, Note::Color)
                        .to(NoteHexColor::Table, NoteHexColor::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade)
                    )
                    // fk
                    .col(integer(Note::Status).not_null())
                    .foreign_key(ForeignKey::create()
                        .name("fk_note_status")
                        .from(Note::Table, Note::Status)
                        .to(NoteStatus::Table, NoteStatus::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade)
                    )
                    .col(
                        ColumnDef::new(Note::CreatedAt)
                            .timestamp()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
                    )
                    .col(
                        ColumnDef::new(Note::UpdatedAt)
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
            .drop_table(Table::drop().table(Note::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Note {
    Table,
    Id,
    Title,
    Detail,
    UserId,
    Color,
    Status,
    CreatedAt,
    UpdatedAt
}
