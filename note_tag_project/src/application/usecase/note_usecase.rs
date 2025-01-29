use std::sync::Arc;

use rocket::serde::json::Json;
use sea_orm::DbErr;

use crate::domain::{dto::note_dto::{ReqCreateNoteDto, ReqUpdateNoteDto, ResNoteEntryDto, ResNoteListDto}, repositories::require_implementation::trait_note_repository::NoteRepository, };

pub struct NoteUseCase<T>
where 
    T: NoteRepository + Send + Sync,
{
    note_repository: Arc<T>
}

impl<T> NoteUseCase<T>
where 
    T: NoteRepository + Send + Sync,
{
    pub async fn new(note_repository: Arc<T>) -> Self {
        Self {
            note_repository: note_repository
        }
    }

    pub async fn create_note(&self, user_id: i32, note_info: ReqCreateNoteDto) -> Result<(), String> {
        let result = self.note_repository.create_note(user_id, note_info).await;
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err("Error creating note".to_string())
        }
    }

    pub async fn get_all_notes(&self, user_id: i32) -> Result<ResNoteListDto, String> {
        
        let result: Result<Vec<ResNoteEntryDto>, DbErr> = self.note_repository.get_all_note(user_id).await;
        match result {
            Ok(notes) => {
                Ok(ResNoteListDto {
                    total: notes.len() as i32,
                    notes: notes
                })
            },
            Err(_) => Err("Error getting notes".to_string())
        }
        
    }

    pub async fn get_note_by_id(&self, user_id: i32, note_id: i32) -> Result<ResNoteEntryDto, String> {
        
        let result = self.note_repository.get_note_by_id(user_id, note_id).await;
        match result {
            Ok(note) => {
                match note {
                    Some(note) => Ok(note),
                    None => Err("Note not found".to_string())
                }
            },
            Err(_) => Err("Error getting note".to_string())
        }

    }

    pub async fn update_note_by_id(&self, user_id: i32 ,note_info: ReqUpdateNoteDto) -> Result<(), String> {
        
        let result: Result<(), DbErr> = self.note_repository.update_note_by_id(user_id, note_info.id ,note_info).await;
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err("Error updating note".to_string())
        }
        
   
    }

    pub async fn delete_note_by_id(&self, user_id: i32, note_id: i32) -> Result<(), String> {
        todo!("Implement delete_note method in NoteUseCase");
        let result = self.note_repository.delete_note_by_id(user_id, note_id).await;
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err("Error deleting note".to_string())
        }
    }
}