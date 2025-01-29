use rocket::response::status;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseTransaction, DbErr, EntityOrSelect, EntityTrait, IntoActiveModel, QueryFilter, Set};
use sea_orm_migration::{async_trait, seaql_migrations::Entity};
use sqlx::types::chrono;
use tracing::info;
use crate::domain::{dto::note_dto::ReqUpdateNoteDto, entities::{note, note_hex_color, note_status, note_tag, prelude::NoteHexColor, tag, user_tag}};





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

    async fn get_color_id_by_color_detail(
        &self,
        txn: &DatabaseTransaction,
        color_detail: &str
    ) -> Result<i32, DbErr> {
        let color = note_hex_color::Entity::find()
            .filter(note_hex_color::Column::HexColor.eq(color_detail))
            .one(txn)
            .await?
            .ok_or(DbErr::RecordNotFound("Failed to fetch color id".to_string()))?.id;
            
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

    async fn get_status_id_by_status_detail(
        &self,
        txn: &DatabaseTransaction,
        status_detail: &str
    ) -> Result<i32, DbErr> {
        let status = note_status::Entity::find()
            .filter(note_status::Column::StatusDetail.eq(status_detail))
            .one(txn)
            .await?
            .ok_or(DbErr::RecordNotFound("Failed to fetch status id".to_string()))?.id;
        
        Ok(status)
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

    