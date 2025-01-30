use std::result::Result;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseTransaction, DbErr, EntityTrait, IntoActiveModel, QueryFilter, Set};
use sea_orm_migration::async_trait;
use sqlx::types::chrono;

use crate::domain::{dto::note_dto::{ReqCreateNoteDto, ReqUpdateNoteDto, ResNoteEntryDto, ResNoteListDto}, entities::note};

use super::{trait_note_hex_color_repository::NoteHexColorRepositoryFullyImplemented, trait_note_status::NoteStatusFullyImplemented};

#[async_trait::async_trait]
pub trait NoteRepository{
    async fn create_note(&self, user_id: i32, note_info: ReqCreateNoteDto) -> Result<(), DbErr>;
    async fn get_note_by_id(&self, user_id: i32, note_id: i32) -> Result<Option<ResNoteEntryDto>, DbErr>;
    async fn get_all_note(&self, user_id: i32) -> Result<Vec<ResNoteEntryDto>, DbErr>;
    async fn update_note_by_id(&self, user_id: i32, note_id: i32, note_info: ReqUpdateNoteDto) -> Result<(), DbErr>;
    async fn delete_note_by_id(&self, user_id: i32, note_id: i32) -> Result<(), DbErr>;
}

#[async_trait::async_trait]
pub trait NoteRepositoryFullyImplemented: NoteHexColorRepositoryFullyImplemented + NoteStatusFullyImplemented {
    async fn update_note_with_option_field(
        &self,
        txn: &DatabaseTransaction,
        note_id: i32,
        note_info: ReqUpdateNoteDto
    ) -> Result<note::Model, DbErr>{
        let note_model = note::Entity::find()
            .filter(note::Column::Id.eq(note_id))
            .one(txn)
            .await?
            .ok_or(DbErr::RecordNotFound("Failed to fetch note".to_string()))?;
        let mut note = note_model.into_active_model();
        if let Some(title) = note_info.title {
            note.title = Set(title);
        }
        if let Some(content) = note_info.content {
            note.detail = Set(content);
        }
        if let Some(color) = note_info.color.filter(|c| !c.is_empty())  {
            let color_id = self.get_color_id_by_color_detail(txn, &color).await?;
            note.color = Set(color_id);
        }
        if let Some(status) = note_info.status.filter(|c| !c.is_empty())  {
            let status_id = self.get_status_id_by_status_detail(txn, &status).await?;
            note.status = Set(status_id);
        }
        note.updated_at = Set(Some(chrono::Local::now().to_utc()));
        let result = note.update(txn).await?;
        
        Ok(result)
    }

    async fn is_user_id_associate_with_note_id(
        &self,
        txn: &DatabaseTransaction,
        user_id: i32,
        note_id: i32,
    ) -> Result<bool, DbErr> {
        let is_associate = note::Entity::find()
            .filter(note::Column::Id.eq(note_id))
            .filter(note::Column::UserId.eq(user_id))
            .one(txn)
            .await?;
        match is_associate {
            Some(_) => Ok(true),
            None => Ok(false)
        }
    }


}