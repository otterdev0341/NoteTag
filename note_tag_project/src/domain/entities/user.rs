//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub username: String,
    pub password: String,
    #[sea_orm(unique)]
    pub email: String,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub gender: i32,
    pub status: i32,
    pub role_id: i32,
    pub created_at: Option<DateTimeUtc>,
    pub updated_at: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::account_status::Entity",
        from = "Column::Status",
        to = "super::account_status::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    AccountStatus,
    #[sea_orm(
        belongs_to = "super::gender::Entity",
        from = "Column::Gender",
        to = "super::gender::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Gender,
    #[sea_orm(has_many = "super::note::Entity")]
    Note,
    #[sea_orm(
        belongs_to = "super::role::Entity",
        from = "Column::RoleId",
        to = "super::role::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Role,
    #[sea_orm(has_many = "super::user_tag::Entity")]
    UserTag,
}

impl Related<super::account_status::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AccountStatus.def()
    }
}

impl Related<super::gender::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Gender.def()
    }
}

impl Related<super::note::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Note.def()
    }
}

impl Related<super::role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Role.def()
    }
}

impl Related<super::user_tag::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserTag.def()
    }
}

impl Related<super::tag::Entity> for Entity {
    fn to() -> RelationDef {
        super::user_tag::Relation::Tag.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::user_tag::Relation::User.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
