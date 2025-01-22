use sea_orm_migration::{prelude::*, schema::*};

use super::{m20220101_000001_create_user_table::User, m20250122_150141_create_tag_table::Tag};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        

        manager
            .create_table(
                Table::create()
                    .table(UserTag::Table)
                        .if_not_exists()
                    .col(integer(UserTag::UserId).not_null())
                    .col(integer(UserTag::TagId).not_null())
                    .col(date_time(UserTag::CreateAt).not_null().default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_tag_user")
                            .from(UserTag::Table, UserTag::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(ForeignKey::create()
                        .name("fk_user_tag_tag")
                        .from(UserTag::Table, UserTag::TagId)
                        .to(Tag::Table, Tag::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade))
                    .primary_key(Index::create()
                        .col(UserTag::UserId)
                        .col(UserTag::TagId))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        

        manager
            .drop_table(Table::drop().table(UserTag::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserTag {
    Table,
    UserId,
    TagId,
    CreateAt,
}
