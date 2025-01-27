use std::sync::Arc;

use rocket::{async_trait, response::status};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set, TransactionTrait};
use sea_orm_migration::async_trait;
use tracing::{error, info};

use crate::domain::{dto::note_dto::{ReqCreateNoteDto, ReqUpdateNoteDto, ResNoteEntryDto}, entities::{note, note_status, note_tag, tag, user, user_tag}, repositories::trait_note_repository::NoteRepository};

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
impl NoteRepository for ImplNoteRepository {
    
    async fn create_note(&self, user_id: i32, note_info: ReqCreateNoteDto) -> Result<(), DbErr> {
        
        // Begin transaction
        let txn = self.db.begin().await?;

        // check is user exist in the database
        let user = match user::Entity::find()
        .filter(user::Column::Id.eq(user_id))
        .one(&txn)
        .await? {
        Some(user) => Some(user),
        None => {
            error!("User not found");
            txn.rollback().await?;
            return Err(DbErr::Custom((format!("The user with id {} not found", user_id)).into()));
            }
        };

        // get {{color id}} by ReqCreateNoteDto from string to i32 to persist to database
        // user can't craete persist new color to database
        // default color id is 1
        let get_color_id = match note::Entity::find()
            .filter(note::Column::Color.eq(note_info.color))
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
        let mut note_tags_id: Vec<i32> = vec![];
        if !note_tags.is_empty() {
            // ** in ths scope each item in note_tags: Vec<String> will call "the_tag"
            // ** in ths scope each item in note_tags_id: Vec<String> will call "the_tag_id"
            // case tag table
                // .1 check if { the_tag } exist in the { tag table } if note "create it" and retrive id, if it exist retrive id
                // .2 then add the id to { note_tags_id: Vec<i32> }
                for the_tag in note_tags{
                    let tag_id = match tag::Entity::find()
                        .filter(tag::Column::TagName.eq(the_tag.clone()))
                        .one(&txn)
                        .await? {
                        Some(tag) => continue,
                        None => {
                            let new_tag = tag::ActiveModel {
                                tag_name: Set(the_tag.clone().to_owned()),
                                ..Default::default()
                            };
                            let inserted_tag = new_tag.insert(&txn).await;
                            match inserted_tag {
                                Ok(tag) => Some(tag.id),
                                Err(err) => {
                                    error!("Error creating tag: {:?}", err);
                                    txn.rollback().await?;
                                    return Err(err);
                                }
                            }
                        }
                    };
                    note_tags_id.push(tag_id.unwrap());
                }
            // case user_tag table
                // .1 check if the tag have an association with the { user_tag_table }
                // .2 if exist skip, if not create it by iterator over { note_tags_id: Vec<i32> }
                for the_tag_id in note_tags_id.clone() {
                    let user_tag_id = match user_tag::Entity::find()
                        .filter(user_tag::Column::UserId.eq(user_id))
                        .filter(user_tag::Column::TagId.eq(the_tag_id))
                        .one(&txn)
                        .await? {
                        Some(user_tag) => continue,
                        None => {
                            let new_user_tag = user_tag::ActiveModel {
                                user_id: Set(user_id),
                                tag_id: Set(the_tag_id),
                                ..Default::default()
                            };
                            let inserted_user_tag = new_user_tag.insert(&txn).await;
                            match inserted_user_tag {
                                Ok(user_tag) => Some(user_tag),
                                Err(err) => {
                                    error!("Error creating user_tag: {:?}", err);
                                    txn.rollback().await?;
                                    return Err(err);
                                }
                            }
                        }
                    };
                }
            // case note_tag table
                // .1 check if the tag_id have an association with the { note_tag_table }
                // .2 if exist skip, if not create it by iterator over { note_tags_id: Vec<i32>
                for the_tag_id in note_tags_id.clone(){
                    let note_tag_id = match note_tag::Entity::find()
                        .filter(note_tag::Column::NoteId.eq(note_id))
                        .filter(note_tag::Column::TagId.eq(the_tag_id))
                        .one(&txn)
                        .await? {
                        Some(_note_tag) => continue,
                        None => {
                            let new_note_tag = note_tag::ActiveModel {
                                note_id: Set(note_id),
                                tag_id: Set(the_tag_id),
                                ..Default::default()
                            };
                            let inserted_note_tag = new_note_tag.insert(&txn).await;
                            match inserted_note_tag {
                                Ok(note_tag) => Some(note_tag),
                                Err(err) => {
                                    error!("Error creating note_tag: {:?}", err);
                                    txn.rollback().await?;
                                    return Err(err);
                                }
                            }
                        }
                    };
                }
        }
        
        

        // Commit transaction
        txn.commit().await?;

        Ok(())
    }

    async fn get_note_by_id(&self, user_id: i32, note_id: i32) -> Result<Option<ResNoteEntryDto>, DbErr> {
        // Implement the function logic here
        Ok(None)
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

