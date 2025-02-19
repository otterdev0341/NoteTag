//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "note")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub title: String,
    pub detail: String,
    pub user_id: i32,
    pub color: i32,
    pub status: i32,
    pub created_at: Option<DateTimeUtc>,
    pub updated_at: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::note_hex_color::Entity",
        from = "Column::Color",
        to = "super::note_hex_color::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    NoteHexColor,
    #[sea_orm(
        belongs_to = "super::note_status::Entity",
        from = "Column::Status",
        to = "super::note_status::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    NoteStatus,
    #[sea_orm(has_many = "super::note_tag::Entity")]
    NoteTag,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    User,
}

impl Related<super::note_hex_color::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::NoteHexColor.def()
    }
}

impl Related<super::note_status::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::NoteStatus.def()
    }
}

impl Related<super::note_tag::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::NoteTag.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::tag::Entity> for Entity {
    fn to() -> RelationDef {
        super::note_tag::Relation::Tag.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::note_tag::Relation::Note.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
