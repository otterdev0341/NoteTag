use sea_orm::{ColumnTrait, DatabaseTransaction, DbErr, EntityTrait, QueryFilter, Set};
use sea_orm_migration::async_trait;
use tracing::info;

use crate::domain::{dto::note_dto::ResNoteEntryDto, entities::{note_tag, tag}};

#[async_trait::async_trait]
pub trait NoteTagRepository {
    async fn get_notes_by_tags(&self, user_id: i32, tags: Vec<String>) -> Result<Vec<ResNoteEntryDto>, DbErr>;
    async fn get_notes_by_keyword(&self, user_id: i32, keyword: String) -> Result<Vec<ResNoteEntryDto>, DbErr>;
}

#[async_trait::async_trait]
pub trait NoteTagRepositoryFullyImplemented {
    // check is { tag_id } is associate with this { Note } or not
    // if not associate will create new association
    // if associate do nothing
    async fn is_tag_id_is_associate_with_note_id_or_create(
        &self,
        txn: &DatabaseTransaction,
        note_id: i32,
        tag_id: i32,
    ) -> Result<note_tag::Model, DbErr> {
        // Check if the association already exists
        if let Some(model) = note_tag::Entity::find()
            .filter(note_tag::Column::NoteId.eq(note_id))
            .filter(note_tag::Column::TagId.eq(tag_id))
            .one(txn)
            .await? 
        {
            // If found, return the existing model
            info!("tag is already associate with note");
            return Ok(model);
        }
    
        // If not found, create a new association
        let new_note_tag = note_tag::ActiveModel {
            note_id: Set(note_id),
            tag_id: Set(tag_id),
            ..Default::default()
        };
    
        // Insert the new association
        let insert_result = note_tag::Entity::insert(new_note_tag)
            .exec(txn)
            .await?;
    
        // Fetch and return the newly inserted record
        let inserted_model = note_tag::Entity::find_by_id(insert_result.last_insert_id)
            .one(txn)
            .await?
            .ok_or(DbErr::RecordNotFound("Failed to fetch inserted note_tag".to_string()))?;
        info!("tag is associate with note with new crated association");
        Ok(inserted_model)
    }

    async fn get_tags_for_note_id(
        &self,
        txn: &DatabaseTransaction,
        note_id: i32,
    ) -> Result<Vec<String>, DbErr> {
        let tags = note_tag::Entity::find()
            .filter(note_tag::Column::NoteId.eq(note_id))
            .all(txn)
            .await?;
        let mut tag_names = Vec::new();
        for tag in tags {
            let tag_name = tag::Entity::find_by_id(tag.tag_id)
                            .one(txn)
                            .await?
                            .ok_or(DbErr::RecordNotFound("Failed to fetch tag name".to_string()))?
                            .tag_name;
            tag_names.push(tag_name);
        }
        Ok(tag_names)
    }

    async fn delete_all_note_tag_relation_by_note_id(
        &self,
        txn: &DatabaseTransaction,
        note_id: i32,
    ) 
    -> Result<u64, DbErr>{
        let result = note_tag::Entity::delete_many()
            .filter(note_tag::Column::NoteId.eq(note_id))
            .exec(txn)
            .await?.rows_affected;
        Ok(result)
    }

}