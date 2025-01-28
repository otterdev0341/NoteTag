use rocket::response::status;
use sea_orm::{ColumnTrait, DatabaseTransaction, DbErr, EntityOrSelect, EntityTrait, QueryFilter, Set};
use sea_orm_migration::async_trait;
use tracing::info;
use crate::domain::entities::{note, note_hex_color, note_status, note_tag, prelude::NoteHexColor, tag, user_tag};





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

    async fn get_color_detail_by_color_id(
        &self,
        txn: &DatabaseTransaction,
        color_id: i32,
    ) -> Result<String, DbErr> {
        let color = note_hex_color::Entity::find()
            .filter(note_hex_color::Column::Id.eq(color_id))
            .one(txn)
            .await?
            .ok_or(DbErr::RecordNotFound("Failed to fetch color detail".to_string()))?.hex_color;
        
        Ok(color)
    }

    async fn get_status_detail_by_status_id(
        &self,
        txn: &DatabaseTransaction,
        status_id: i32,
    ) -> Result<String, DbErr> {
        let status = note_status::Entity::find()
            .filter(note_status::Column::Id.eq(status_id))
            .one(txn)
            .await?
            .ok_or(DbErr::RecordNotFound("Failed to fetch status detail".to_string()))?.status_detail;
        
        Ok(status)
    }
    
    
}

    