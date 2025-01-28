use std::result;

use sea_orm::{ColumnTrait, DatabaseTransaction, DbErr, EntityTrait, QueryFilter, Set};
use sea_orm_migration::async_trait;
use tracing::info;
use crate::domain::entities::{note_tag, tag, user_tag};





#[async_trait::async_trait]
pub trait AssociationTagHelperFullyImplemented {

    // check is tag exist in tag table or not
    // if exist will return id of tag
    // if not exist will create new tag and return id of tag
    // it casesensitive that menn "hello" not same with "Hello"
    async fn is_this_tag_is_exist_in_tag_table_or_create(
        &self,
        txn: &DatabaseTransaction,
        user_tag: &str,
    ) -> Result<i32, DbErr> {
        let is_tag_exist = tag::Entity::find()
            .filter(tag::Column::TagName.eq(user_tag))
            .one(txn)
            .await?;
        let tag_id = match is_tag_exist {
            Some(tag) => tag.id,
            None => {
                let new_tag = tag::ActiveModel {
                    tag_name: Set(user_tag.to_string()),
                    ..Default::default()
                };
                let tag_id = tag::Entity::insert(new_tag)
                    .exec(txn)
                    .await?
                    .last_insert_id;
                tag_id
            }
        };
        Ok(tag_id)
    }


    // check is { tag_id } is associate with this { user } or not
    // if not associate will create new association
    // if associate do nothing
    async fn is_tag_id_is_associate_with_this_user_or_create(
        &self,
        txn: &DatabaseTransaction,
        user_id: i32,
        tag_id: i32,
    ) -> Result<user_tag::Model, DbErr> 
    {
        let is_tag_associate = user_tag::Entity::find()
            .filter(user_tag::Column::UserId.eq(user_id))
            .filter(user_tag::Column::TagId.eq(tag_id))
            .one(txn)
            .await?;
        match is_tag_associate {
            Some(model) => {
                info!("tag is already associate with user");
                Ok(model)
            },
            None => {
                let new_user_tag = user_tag::ActiveModel {
                    user_id: Set(user_id),
                    tag_id: Set(tag_id),
                    ..Default::default()
                };
                let insert_result = user_tag::Entity::insert(new_user_tag)
                    .exec(txn)
                    .await?;
                let inserted_model = user_tag::Entity::find_by_id(insert_result.last_insert_id)
                    .one(txn)
                    .await?
                    .ok_or(DbErr::RecordNotFound("Failed to fetch inserted note_tag".to_string()))?;
                info!("tag is associate with user");
                Ok(inserted_model)
            }
        }
    
    }

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
    
}

    