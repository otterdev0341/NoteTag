use sea_orm_migration::{prelude::*, schema::*};

use super::{m20250122_133902_craete_role_table::Role, m20250122_135412_craete_account_status_table::AccountStatus, m20250122_141508_create_gender_table::Gender};


#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(pk_auto(User::Id).integer())
                    .col(string_len(User::Username,50).not_null().unique_key())
                    .col(string_len(User::Password,80).not_null())
                    .col(string(User::Email).not_null().unique_key())
                    .col(string_len(User::FirstName,80).not_null())
                    .col(string_len(User::MiddleName,80).not_null())
                    .col(string_len(User::LastName,80).not_null())
                    //fk - done
                    .col(integer(User::Gender).not_null())
                    .foreign_key(ForeignKey::create()
                        .name("fk_user_gender")
                        .from(User::Table, User::Gender)
                        .to(Gender::Table, Gender::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade)
                    )
                    //fk - done
                    .col(integer(User::Status).not_null())
                    .foreign_key(ForeignKey::create()
                        .name("fk_user_status")
                        .from(User::Table, User::Status)
                        .to(AccountStatus::Table, AccountStatus::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade)
                    )
                    //fk - done
                    .col(integer(User::RoleId).not_null())
                    .foreign_key(ForeignKey::create()
                        .name("fk_user_role")
                        .from(User::Table, User::RoleId)
                        .to(Role::Table, Role::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade))
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .timestamp()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
                    )
                    .col(
                        ColumnDef::new(User::UpdatedAt)
                            .timestamp()
                            .extra("DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP".to_owned()),
                    )
                    .to_owned()
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        

        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum User {
    Table,
    Id,
    Username,
    Password,
    Email,
    FirstName,
    MiddleName,
    LastName,
    Gender,
    Status,
    RoleId,
    CreatedAt,
    UpdatedAt
}
