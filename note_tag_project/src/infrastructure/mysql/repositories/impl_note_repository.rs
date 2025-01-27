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
            // start for
            for tag in note_tags {
                // skip if it is empty string
                if tag.trim().is_empty() {
                    continue;
                }
                // check if the tag exist in the tag table if note create it and retrive id, if it exist retrive id
                let each_tag_id = match tag::Entity::find()
                    .filter(tag::Column::TagName.eq(&tag))
                    .one(&txn)
                    .await? {
                    Some(tag) => Some(tag.id),
                    None => {
                        let new_tag = tag::ActiveModel {
                            tag_name: Set(tag),
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
                // check if the tag have an association with the user_tag_table
                // if exist skip, if not create it
                let user_tag = user_tag::Entity::find()
                    .filter(tag::Column::Id.eq(each_tag_id.unwrap()))
                    .filter(user::Column::Id.eq(user_id))
                    .one(&txn)
                    .await?;
                if user_tag.is_none() {
                    let new_user_tag = user_tag::ActiveModel {
                        user_id: Set(user_id),
                        tag_id: Set(each_tag_id.unwrap()),
                        ..Default::default()
                    };
                    let inserted_user_tag = new_user_tag.insert(&txn).await;
                    match inserted_user_tag {
                        Ok(user_tag) => {
                            info!("User tag created: {:?}", user_tag);
                        },
                        Err(err) => {
                            error!("Error creating user tag: {:?}", err);
                            txn.rollback().await?;
                            return Err(err);
                        }
                    }
                }
                // check if the tag_id have an association with the note_tag_table
                // if exist skip, if not create it
                let note_tag = note::Entity::find()
                    .filter(tag::Column::Id.eq(each_tag_id.unwrap()))
                    .filter(note::Column::Id.eq(note_id))
                    .one(&txn)
                    .await?;
                if note_tag.is_none() {
                    let new_note_tag = note_tag::ActiveModel {
                        note_id: Set(note_id),
                        tag_id: Set(each_tag_id.unwrap()),
                        ..Default::default()
                    };
                    let inserted_note_tag = new_note_tag.insert(&txn).await;
                    match inserted_note_tag {
                        Ok(note_tag) => {
                            info!("Note tag created: {:?}", note_tag);
                        },
                        Err(err) => {
                            error!("Error creating note tag: {:?}", err);
                            txn.rollback().await?;
                            return Err(err);
                        }
                    }
                }
            }
            
            // end for
        // it comming as Vec<String> so we need to iterate over it to check and create
        // then add relation from note and tag to join table

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

