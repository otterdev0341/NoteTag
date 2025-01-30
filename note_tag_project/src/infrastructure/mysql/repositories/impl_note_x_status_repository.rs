use std::sync::Arc;


use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, IntoActiveModel, QueryFilter, Set, TransactionTrait};
use sea_orm_migration::async_trait;
use sqlx::types::chrono;

use crate::domain::{entities::note, repositories::require_implementation::{trait_note_hex_color_repository::NoteHexColorRepositoryFullyImplemented, trait_note_repository::{NoteRepository, NoteRepositoryFullyImplemented}, trait_note_status::{NoteStatusRepository, NoteStatusRepositoryFullyImplemented}, trait_user_repository::UserRepositoryFullyImplemented}};



pub struct ImplNoteStatusRepository {
    pub db : Arc<DatabaseConnection>
}

impl ImplNoteStatusRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        ImplNoteStatusRepository {
            db
        }
    }
}


impl NoteRepositoryFullyImplemented for ImplNoteStatusRepository {}

impl NoteStatusRepositoryFullyImplemented for ImplNoteStatusRepository {}

impl NoteHexColorRepositoryFullyImplemented for ImplNoteStatusRepository {}

#[async_trait::async_trait]
impl NoteStatusRepository for ImplNoteStatusRepository {
    async fn toggle_note_status(&self, user_id: i32, note_id: i32) -> Result<(), DbErr> {
        let txn = self.db.begin().await?;
        
        let is_associate = self.is_user_id_associate_with_note_id(&txn, user_id, note_id).await?;
        if !is_associate {
            return Err(DbErr::RecordNotFound("Note not found".to_string()));
        }
        
        let note_model = note::Entity::find()
            .filter(note::Column::Id.eq(note_id))
            .one(&txn)
            .await?
            .ok_or(DbErr::RecordNotFound("Failed to fetch note".to_string()))?;
        
        let mut note = note_model.into_active_model();
        
        // Extract the value from ActiveValue
        let current_status = match note.status {
            ActiveValue::Set(val) => val,
            ActiveValue::Unchanged(val) => val,
            _ => 1, // Default fallback
        };
    
        let status_id = match current_status {
            1 => 2,
            2 => 1,
            _ => 1,
        };
    
        note.status = Set(status_id);
        note.updated_at = Set(Some(chrono::Local::now().to_utc()));
        note.update(&txn).await?;
        txn.commit().await?;
        
        Ok(())
    }
    
}