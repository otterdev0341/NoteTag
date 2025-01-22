
use sea_orm_migration::{prelude::*, schema::*};

use super::{m20250122_150141_create_tag_table::Tag, m20250122_151252_create_note_table::Note};



#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        

        manager
            .create_table(
                Table::create()
                    .table(NoteTag::Table)
                        .if_not_exists()
                    .col(integer(NoteTag::NoteId).not_null())
                    .col(integer(NoteTag::TagId).not_null())
                    .col(date_time(NoteTag::CreateAt).not_null().default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_note_id")
                            .from(NoteTag::Table, NoteTag::NoteId)
                            .to(Note::Table, Note::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                        .name("fk_tag_id")
                        .from(NoteTag::Table, NoteTag::TagId)
                        .to(Tag::Table, Tag::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade))
                    .primary_key(Index::create()
                        .col(NoteTag::NoteId)
                        .col(NoteTag::TagId))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        

        manager
            .drop_table(Table::drop().table(NoteTag::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum NoteTag {
    Table,
    NoteId,
    TagId,
    CreateAt,
}
