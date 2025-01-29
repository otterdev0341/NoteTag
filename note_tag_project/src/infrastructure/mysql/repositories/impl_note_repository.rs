use std::sync::Arc;

use rocket::response::status;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityOrSelect, EntityTrait, IntoActiveModel, ModelTrait, QueryFilter, Set, TransactionTrait};

use sea_orm_migration::async_trait;
use tracing::{error, info};
use utoipa::openapi::tag;

use crate::domain::{dto::note_dto::{ReqCreateNoteDto, ReqUpdateNoteDto, ResNoteEntryDto}, entities::{note, note_hex_color, note_status}, repositories::{fully_implemented::{trait_association_helper_fully::AssociationTagHelperFullyImplemented, trait_entity_helper_fully::EntityHelperFullyImplemented}, require_implementation::trait_note_repository::NoteRepository}};

pub struct ImplNoteRepository {
    pub db: Arc<DatabaseConnection>
}


impl ImplNoteRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        ImplNoteRepository {
            db
        }
    }

}

#[async_trait::async_trait]
impl EntityHelperFullyImplemented for ImplNoteRepository {
    // use complete funtion that already implement in Trait EntityHelper,
    // because we need to use this function in this repository
    // and use without implement it in this repository
}

#[async_trait::async_trait]
impl AssociationTagHelperFullyImplemented for ImplNoteRepository {
    // use complete funtion that already implement in Trait AssociationTagHelper,
    // because we need to use this function in this repository
    // and use without implement it in this repository
}


#[async_trait::async_trait]
impl NoteRepository for ImplNoteRepository {
    
    async fn create_note(&self, user_id: i32, note_info: ReqCreateNoteDto) -> Result<(), DbErr> {
        
        // Begin transaction
        let txn = self.db.begin().await?;

        // check is status is active
        // the use must be exist if not, authentification middleware should be handle this
        let is_user_active = self.is_user_status_is_active(&txn, user_id).await?;
        match is_user_active {
            false => {
                error!("User is not active");
                txn.rollback().await?;
                return Err(DbErr::Custom(format!("User is not active, please contact admin")));
            },
            _ => {}
        };

        // get {{color id}} by ReqCreateNoteDto from string to i32 to persist to database
        // user can't craete persist new color to database
        // default color id is 1
        let get_color_id = match note_hex_color::Entity::find()
            .filter(note_hex_color::Column::HexColor.eq(note_info.color))
            .one(&txn)
            .await? {
            Some(color) => Some(color.id),
            None => {
                Some(1)
            }
            };
        // get {{Note status}} id by ReqCreateNoteDto from string to i32 to persist to database
        // user can't craete persist new status to database
        // default status id is 1 = unpin
        // note color comming as string (#aabb7755) need to find the id from the database
        let get_status_id =  match note_status::Entity::find()
            .filter(note_status::Column::StatusDetail.eq(note_info.status))
            .one(&txn)
            .await? {
            Some(status) => Some(status.id),
            None => {
                Some(1)
            }
            };
        // persist new note to database to get {{note id}}
        let new_note = note::ActiveModel {
            title: Set(note_info.title.unwrap().to_owned()),
            detail: Set(note_info.content.unwrap().to_owned()),
            user_id: Set(user_id),
            color: Set(get_color_id.unwrap()),
            status: Set(get_status_id.unwrap()),
            ..Default::default()
        };
        let inserted_note = new_note.insert(&txn).await;
        
        let success_inserted = match inserted_note {
            Ok(note) => {
                note
            },
            Err(err) => {
                error!("Error creating note: {:?}", err);
                txn.rollback().await?;
                return Err(err);
            }
        };

        // use note id to persist note tags to database from ReqCreateNoteDto
        let note_id = success_inserted.id;
        let note_tags = note_info.noteTags.unwrap_or_default();
        
        
        // check if the note tag is not empty
        // mean we need to handle relation with tag_table, and user_tag_table and note_tag_table
        
        if !note_tags.is_empty() {
            // ** in ths scope each item in note_tags: Vec<String> will call "the_tag"
            // ** in ths scope each item in note_tags_id: Vec<String> will call "the_tag_id"
            for the_tag in note_tags {
                // case tag table
                // .1 check if { the_tag } exist in the { tag table } if note "create it" and retrive id, if it exist retrive id
                let the_tag_id = self.is_this_tag_is_exist_in_tag_table_or_create(&txn, &the_tag).await?;
                // case user_tag table
                // .1 check if the tag have an association with the { user_tag_table }
                // .2 if exist skip, if not create it by iterator over { note_tags_id: Vec<i32> }
                let user_tag_associate = self.is_tag_id_is_associate_with_this_user_or_create(&txn, user_id, the_tag_id).await;
                match user_tag_associate {
                    Ok(_) => {},
                    Err(err) => {
                        error!("Error creating user tag: {:?}", err);
                        txn.rollback().await?;
                        return Err(err);
                    }
                }
                // case note_tag table
                // .1 check if the tag_id have an association with the { note_tag_table }
                // .2 if exist skip, if not create it by iterator over { note_tags_id: Vec<i32>
                let note_tag_associate = self.is_tag_id_is_associate_with_note_id_or_create(&txn, note_id, the_tag_id).await;
                match note_tag_associate {
                    Ok(_) => {},
                    Err(err) => {
                        error!("Error creating note tag: {:?}", err);
                        txn.rollback().await?;
                        return Err(err);
                    }
                }
            }       
        }
        
        // Commit transaction
        txn.commit().await?;
        Ok(())
        

    }

    async fn get_note_by_id(&self, user_id: i32, note_id: i32) -> Result<Option<ResNoteEntryDto>, DbErr> {
        // Begin transaction
        let txn = self.db.begin().await?;
    
        // is_user_active check
        let is_user_active = self.is_user_status_is_active(&txn, user_id).await?;
        if !is_user_active {
            error!("User is not active");
            txn.rollback().await?;
            return Err(DbErr::Custom("User is not active, please contact admin".to_string()));
        }
    
        // Check if the note is associated with the user
        let found_note = note::Entity::find()
            .filter(note::Column::Id.eq(note_id))
            .filter(note::Column::UserId.eq(user_id))
            .one(&txn)
            .await?;
    
        match found_note {
            Some(note) => {
                // Fetch associated tags for the note
                let tags_for_note = self.get_tags_for_note_id(&txn, note_id).await?;
                let color = self.get_color_detail_by_color_id(&txn, note.color).await?;
                let status = self.get_status_detail_by_status_id(&txn, note.status).await?;
                // Prepare the response DTO
                let return_note = ResNoteEntryDto {
                    id: note.id,
                    title: note.title.clone(),
                    content: note.detail.clone(),
                    colorCode: color,
                    status: status,
                    noteTags: tags_for_note,
                    createdAt: note.created_at.map(|dt| dt.to_string()).unwrap_or_default(),
                    updatedAt: note.updated_at.map(|dt| dt.to_string()).unwrap_or_default(),
                };
    
                // Return the response
                Ok(Some(return_note))
            },
            None => {
                // Handle case where note is not found
                error!("Note is not associated with user");
                txn.rollback().await?;
                Err(DbErr::Custom("Note is not associated with user".to_string()))
            },
        }
    }

    async fn get_all_note(&self, user_id: i32) -> Result<Vec<ResNoteEntryDto>, DbErr> {
        
        // begin trasaction
        let txn = self.db.begin().await?;
        // check is user active
        let is_user_active = self.is_user_status_is_active(&txn, user_id).await?;
        if !is_user_active {
            error!("User is not active");
            txn.rollback().await?;
            return Err(DbErr::Custom("User is not active, please contact admin".to_string()));
        }
        
        // get all note by user_id
        let all_notes = note::Entity::find()
            .filter(note::Column::UserId.eq(user_id))
            .all(&txn)
            .await?;

        // prepare the response dto
        let mut return_notes = Vec::new();
        for note in all_notes {
            let tags_for_note = self.get_tags_for_note_id(&txn, note.id).await?;
            let color = self.get_color_detail_by_color_id(&txn, note.color).await?;
            let status = self.get_status_detail_by_status_id(&txn, note.status).await?;
            let temp_note = ResNoteEntryDto {
                id: note.id,
                title: note.title.clone(),
                content: note.detail.clone(),
                colorCode: color,
                status: status,
                noteTags: tags_for_note,
                createdAt: note.created_at.map(|dt| dt.to_string()).unwrap_or_default(),
                updatedAt: note.updated_at.map(|dt| dt.to_string()).unwrap_or_default(),
            };
            return_notes.push(temp_note);
        }
        // commit transaction
        txn.commit().await?;
        Ok(return_notes)
    }

    async fn update_note_by_id(&self, user_id: i32, note_id: i32, note_info: ReqUpdateNoteDto) -> Result<(), DbErr> {
        // begin trasaction
        let txn = self.db.begin().await?;
        // check is user_active
        let is_user_active = self.is_user_status_is_active(&txn, user_id).await?;
        if !is_user_active {
            error!("User is not active");
            txn.rollback().await?;
            return Err(DbErr::Custom("User is not active, please contact admin".to_string()));
        }
        // check is note_id is associate with user_id
        let note_association = self.is_user_id_associate_with_note_id(&txn, user_id, note_id).await?;
        if !note_association {
            error!("Note is not associate with user");
            txn.rollback().await?;
            return Err(DbErr::Custom("Note is not associate with user".to_string()));
        }
        // Persist update note phase
        // 1. update only the field that is not None
        let update_note = self.update_note_with_option_field(&txn, note_id, note_info.clone()).await?;
        
        // 2. update the tag
            // tag can be muntiple, so we need to check if the tag is exist or not or create it
            // old is : ["cat","dog"] and new is : ["cat","bat", "otter"]
            // mean need to check new tag is exist or not, if not create it
            // and remove old note_tag and create a new for the new tag
            // 2.1 drop all note_tag that associate with the note_id
        self.delete_all_note_tag_relation_by_note_id(&txn, note_id).await?;
            // 2.2 create and relate new note_tag with the note_id
        for tag in note_info.noteTags.unwrap() {
            
            if tag.trim().is_empty() {
                continue;
            }
            
            let tag_id = self.is_this_tag_is_exist_in_tag_table_or_create(&txn, &tag).await?;
            let user_tag_associate = self.is_tag_id_is_associate_with_this_user_or_create(&txn, user_id, tag_id).await?;
            let note_tag_associate = self.is_tag_id_is_associate_with_note_id_or_create(&txn, note_id, tag_id).await?;
        }
        // commit transaction
        txn.commit().await?;
        Ok(())
    }

    async fn delete_note_by_id(&self, user_id: i32, note_id: i32) -> Result<(), DbErr> {
        // begin trasaction
        let txn = self.db.begin().await?;
        // is user_active
        let is_user_active = self.is_user_status_is_active(&txn, user_id).await?;
        if !is_user_active {
            error!("User is not active");
            txn.rollback().await?;
            return Err(DbErr::Custom("User is not active, please contact admin".to_string()));
        }
        // is note_id associate with user_id
        let note_association = self.is_user_id_associate_with_note_id(&txn, user_id, note_id).await?;
        if note_association {
            let note_delete = note::Entity::find()
                .filter(note::Column::Id.eq(note_id))
                .one(&txn)
                .await?;
            match note_delete {
                Some(note) => {
                    let delete_result = note.delete(&txn).await;
                    match delete_result {
                        Ok(_) => {},
                        Err(err) => {
                            error!("Error deleting note: {:?}", err);
                            txn.rollback().await?;
                            return Err(err);
                        }
                    }
                },
                None => {
                    error!("Note is not associate with user");
                    txn.rollback().await?;
                    return Err(DbErr::Custom("Note is not associate with user".to_string()));
                }
            }
        }else{
            return Err(DbErr::Custom("Note is not associate with user".to_string()));
        }
        // commit transaction
        txn.commit().await?;
        Ok(())
    }
}

