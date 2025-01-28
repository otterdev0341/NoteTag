use std::sync::Arc;

use rocket::response::status;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityOrSelect, EntityTrait, QueryFilter, Set, TransactionTrait};

use sea_orm_migration::async_trait;
use tracing::{error, info};

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
        // Implement the function logic here
        Ok(vec![])
    }

    async fn update_note_by_id(&self, user_id: i32, note_id: i32, note_info: ReqUpdateNoteDto) -> Result<(), DbErr> {
        // Implement the function logic here
        Ok(())
    }

    async fn delete_note_by_id(&self, user_id: i32, note_id: i32) -> Result<(), DbErr> {
        // Implement the function logic here
        Ok(())
    }
}

