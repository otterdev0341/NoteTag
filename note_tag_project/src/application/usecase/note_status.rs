use std::sync::Arc;



use crate::domain::repositories::require_implementation::trait_note_status::NoteStatusRepository;

pub struct NoteStatusUseCase<T>
where
    T: NoteStatusRepository + Send + Sync,
{
    note_status_repository: Arc<T>,
}


impl<T> NoteStatusUseCase<T>
where
    T: NoteStatusRepository + Send + Sync,
{
    pub async fn new(note_status_repository: Arc<T>) -> Self {
        Self {
            note_status_repository: note_status_repository,
        }
    }

    pub async fn toggle_note_status(&self, user_id: i32, note_id: i32) -> Result<(), String> {
        let result = self
            .note_status_repository
            .toggle_note_status(user_id, note_id)
            .await;
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err("Error toggling note status".to_string()),
        }
    }
}