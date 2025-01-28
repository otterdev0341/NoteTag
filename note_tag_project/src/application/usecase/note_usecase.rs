use std::sync::Arc;

use crate::domain::{dto::note_dto::{ReqCreateNoteDto, ReqUpdateNoteDto, ResNoteEntryDto}, repositories::require_implementation::trait_note_repository::NoteRepository, };

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

    pub async fn get_all_notes(&self, user_id: i32) -> Result<Vec<ResNoteEntryDto>, String> {
        todo!("Implement get_notes method in NoteUseCase");
        let result = self.note_repository.get_all_note(user_id).await;
        match result {
            Ok(notes) => Ok(notes),
            Err(_) => Err("Error getting notes".to_string())
        }
    }

    pub async fn get_note_by_id(&self, user_id: i32, note_id: i32) -> Result<ResNoteEntryDto, String> {
        todo!("Implement get_note_by_id method in NoteUseCase");
        let result = self.note_repository.get_note_by_id(user_id, note_id).await;

    }

    pub async fn update_note_by_id(&self, user_id: i32, note_info: ReqUpdateNoteDto) -> Result<(), String> {
        todo!("Implement update_note method in NoteUseCase");
        
   
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