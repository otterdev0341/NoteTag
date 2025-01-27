use std::result::Result;
use sea_orm::DbErr;
use sea_orm_migration::async_trait;

use crate::domain::dto::note_dto::{ReqCreateNoteDto, ReqUpdateNoteDto, ResNoteEntryDto};

#[async_trait::async_trait]
pub trait NoteRepository{
    async fn create_note(&self, user_id: i32, note_info: ReqCreateNoteDto) -> Result<(), DbErr>;
    async fn get_note_by_id(&self, user_id: i32, note_id: i32) -> Result<Option<ResNoteEntryDto>, DbErr>;
    async fn get_all_note(&self, user_id: i32) -> Result<Vec<ResNoteEntryDto>, DbErr>;
    async fn update_note_by_id(&self, user_id: i32, note_id: i32, note_info: ReqUpdateNoteDto) -> Result<(), DbErr>;
    async fn delete_note_by_id(&self, user_id: i32, note_id: i32) -> Result<(), DbErr>;
}