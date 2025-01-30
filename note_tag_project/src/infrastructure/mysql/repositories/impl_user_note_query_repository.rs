use std::{collections::HashSet, sync::Arc};

use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, JoinType, QueryFilter, QuerySelect, TransactionTrait};
use sea_orm_migration::async_trait;

use crate::domain::{dto::{note_dto::ResNoteEntryDto, query::QueryNoteDto}, entities::{note, note_tag, prelude::NoteStatus, tag}, repositories::require_implementation::{trait_note_hex_color_repository::NoteHexColorRepositoryFullyImplemented, trait_note_status::NoteStatusRepositoryFullyImplemented, trait_note_x_tag_repository::NoteTagRepositoryFullyImplemented, trait_user_note_query_repository::UserNoteQueryRepository, trait_user_repository::UserRepositoryFullyImplemented}};

pub struct ImplUserNoteQueryRepository{
    pub db: Arc<DatabaseConnection>
}

impl ImplUserNoteQueryRepository{
    pub fn new(db: Arc<DatabaseConnection>) -> Self{
        ImplUserNoteQueryRepository{
            db
        }
    }
}

#[async_trait::async_trait]
impl UserRepositoryFullyImplemented for ImplUserNoteQueryRepository{}

#[async_trait::async_trait]
impl NoteTagRepositoryFullyImplemented for ImplUserNoteQueryRepository{}

#[async_trait::async_trait]
impl NoteStatusRepositoryFullyImplemented for ImplUserNoteQueryRepository{}

#[async_trait::async_trait]
impl NoteHexColorRepositoryFullyImplemented for ImplUserNoteQueryRepository{}

#[async_trait::async_trait]
impl UserNoteQueryRepository for ImplUserNoteQueryRepository{
    async fn query_notes(&self, user_id: i32, query_info: QueryNoteDto) -> Result<Vec<ResNoteEntryDto>, DbErr>{
        // begin trasaction
        let txn = self.db.begin().await?;
        // check is user_id valid
        let is_user_active = self.is_user_status_is_active(&txn, user_id).await?;
        if !is_user_active{
            return Err(DbErr::RecordNotFound("User not found".to_string()));
        }
        
        // start query
        let mut query = note::Entity::find();
        
        // apply filter from query_info on title
        if let Some(title) = query_info.title.as_deref()
                    .map(str::trim)
                    .filter(|c| !c.is_empty())
        {
            query = query.filter(note::Column::Title.contains(title));
        }
        // apply filter from query_info on content
        if let Some(content) = query_info.content.as_deref()
                    .map(str::trim)
                    .filter(|c| !c.is_empty())
        {
            query = query.filter(note::Column::Detail.contains(content));
        }

        // handle Vec<String> that collect all citeria tags
        if let Some(tags) = query_info.noteTags {
            let valid_tags: Vec<String> = tags
                .into_iter()
                .map(|c| c.trim().to_string())
                .filter(|c| !c.is_empty())
                .collect::<HashSet<_>>()
                .into_iter()
                .collect();
            if !valid_tags.is_empty(){
                query = query
                    .join(JoinType::LeftJoin, note::Entity::has_many(note_tag::Entity).into()) 
                    .join(JoinType::LeftJoin, note_tag::Entity::belongs_to(tag::Entity).into()) 
                    .filter(tag::Column::TagName.is_in(valid_tags));
            }
        }
        // query all note
        let notes = query.all(&txn).await?; 

        // map to Vec<ResNoteEntryDto>
        let mut result = Vec::new();
        for note in notes{
            let tags = self.get_tags_for_note_id(&txn, note.id).await?;
            let status = self.get_status_detail_by_status_id(&txn, note.status).await?;
            let color = self.get_color_detail_by_color_id(&txn, note.color).await?;
            result.push(ResNoteEntryDto{
                id: note.id,
                title: note.title.clone(),
                content: note.detail.clone(),
                colorCode: color,
                status: status,
                noteTags: tags,
                createdAt: note.created_at.map(|dt| dt.to_string()).unwrap_or_default(),
                updatedAt: note.updated_at.map(|dt| dt.to_string()).unwrap_or_default(),
            });
        }
        // commit transaction
        txn.commit().await?;
        Ok(result)
    }
}

