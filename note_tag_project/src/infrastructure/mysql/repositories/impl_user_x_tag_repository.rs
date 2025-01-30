use std::sync::Arc;

use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, ModelTrait, QueryFilter, Set, TransactionTrait};
use sea_orm_migration::async_trait;
use tracing::{error, info};

use crate::domain::{entities::{self, tag, user, user_tag}, repositories::require_implementation::trait_user_x_tag_repository::{UserTagRepository, UserTagRepositoryFullyImplementd}};

pub struct ImplUserTagRepository {
    pub db : Arc<DatabaseConnection>
}

impl ImplUserTagRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        ImplUserTagRepository {
            db
        }
    }
}

impl UserTagRepositoryFullyImplementd for ImplUserTagRepository {
    
}


#[async_trait::async_trait]
impl UserTagRepository for ImplUserTagRepository {

    async fn create_user_tag(&self, user_id: i32, tag_name: &str) -> Result<(), DbErr> {
        
        let txn = self.db.begin().await?;
        
            // Check if the tag already exists
        let tag_model = tag::Entity::find()
        .filter(tag::Column::TagName.eq(tag_name))
        .one(&txn)
        .await?;
        
        // Insert the tag if it doesn't exist
        let tag_id = if let Some(tag) = tag_model {
                tag.id
            } else {
                let new_tag = tag::ActiveModel {
                    tag_name: Set(tag_name.to_string()),
                    ..Default::default()
                };
                let inserted_tag = new_tag.insert(&txn).await?;
                inserted_tag.id
            };
        // Link the user and the tag in the join table
        let user_x_tag_entry = user_tag::ActiveModel {
        user_id: Set(user_id),
        tag_id: Set(tag_id),
        ..Default::default()
        };
        user_x_tag_entry.insert(&txn).await?;
        txn.commit().await?;
        Ok(())    
    }
   
    

    async fn get_user_tags(&self, user_id: i32) -> Result<Vec<String>, DbErr> {
   
        // find the user
        let user = user::Entity::find_by_id(user_id).one(&*self.db).await;
        match user {
            Ok(user) => {
                // find the tags associated with the user
                if let Some(user) = user {
                    let tags = user.find_related(tag::Entity).all(&*self.db).await;
                    match tags {
                        Ok(tags) => {
                            let tag_names = tags.iter().map(|tag| tag.tag_name.clone()).collect();
                            Ok(tag_names)
                        }
                        Err(e) => Err(e),
                    }
                } else {
                    Ok(vec![])
                }
            }
            Err(e) => Err(e),
        }
    }
    // END :


    async fn update_user_tag(&self, user_id: i32, old_tag: &str, new_tag: &str) -> Result<(), DbErr> {
        let the_old_tag = String::from(old_tag);
        let the_new_tag = String::from(new_tag);
    
        // Begin the transaction
        let txn = self.db.begin().await?;
    
        // Step 1: Validate the user exists and retrieve the user id.
        let valid_user = user::Entity::find()
            .filter(user::Column::Id.eq(user_id))
            .one(&txn)
            .await?;
        let the_user_id = match valid_user {
            Some(user) => user.id,
            None => {
                error!("User with id {} not found", user_id);
                txn.rollback().await?;  // Rollback transaction
                return Err(DbErr::Custom(format!("User with id {} not found", user_id)));
            }
        };
    
        // Step 2: Check if the old tag exists in the database
        let old_tag = tag::Entity::find()
            .filter(tag::Column::TagName.eq(&the_old_tag))
            .one(&txn)
            .await?;
        let the_old_tag_id = match old_tag {
            Some(tag) => tag.id,
            None => {
                error!("Tag with name {} not found", the_old_tag);
                txn.rollback().await?;  // Rollback transaction
                return Err(DbErr::Custom(format!("Can't find tag id for tag {}", the_old_tag)));
            }
        };
    
        // Step 3: Check if the user-tag association exists
        let user_tag_association: Option<user_tag::Model> = user_tag::Entity::find()
            .filter(user_tag::Column::UserId.eq(the_user_id))
            .filter(user_tag::Column::TagId.eq(the_old_tag_id))
            .one(&txn)
            .await?;
        let result_find = match user_tag_association {
            Some(user_tag) => user_tag,
            None => {
                error!("User with id {} and tag with id {} not found", the_user_id, the_old_tag_id);
                txn.rollback().await?;  // Rollback transaction
                return Err(DbErr::Custom(format!("User with id {} and tag with id {} not found in association table", the_user_id, the_old_tag_id)));
            }
        };
    
        // Step 4: Find or create the new tag
        let the_new_tag_id = match tag::Entity::find()
            .filter(tag::Column::TagName.eq(&the_new_tag))
            .one(&txn)
            .await? {
                Some(tag) => tag.id,
                None => {
                    let new_tag = tag::ActiveModel {
                        tag_name: Set(the_new_tag),
                        ..Default::default()
                    };
                    let inserted_tag = new_tag.insert(&txn).await?;
                    inserted_tag.id
                }
            };
    
        // Step 5: Update the drop the association and create a new one
        let _delete_result = result_find.delete(&txn).await;
        let new_record = user_tag::ActiveModel {
            user_id: Set(the_user_id),
            tag_id: Set(the_new_tag_id),
            ..Default::default()
        };
        new_record.insert(&txn).await?;
        // Commit the transaction
        let commit_result = txn.commit().await;
        match commit_result {
            Ok(_) => {
                info!("Transaction committed successfully");
                Ok(())
            }
            Err(e) => {
                error!("Error committing transaction: {}", e);
                Err(DbErr::Custom(format!("Error committing transaction: {}", e)))
            }
        }
    }
    
    

    async fn delete_tag_from_user(&self, user_id: i32, tag_name: &str) -> Result<(), DbErr> {
        
        // Begin the transaction
        let txn = self.db.begin().await?;

        // check is user valid
        let _user = match user::Entity::find()
            .filter(user::Column::Id.eq(user_id))
            .one(&txn)
            .await? {
                Some(user) => user,
                None => {
                    error!("User with id {} not found", user_id);
                    txn.rollback().await?;  // Rollback transaction
                    return Err(DbErr::Custom(format!("User with id {} not found", user_id)));
                }
            };
        
        // check is tag valid in tag table
        let tag = match tag::Entity::find()
            .filter(tag::Column::TagName.eq(tag_name))
            .one(&txn)
            .await? {
                Some(tag) => tag,
                None => {
                    error!("Tag with name {} not found", tag_name);
                    txn.rollback().await?;  // Rollback transaction
                    return Err(DbErr::Custom(format!("Tag with name {} not found", tag_name)));
                }
            };
        // check is tag have associate with the user
        let user_tag = match user_tag::Entity::find()
            .filter(user_tag::Column::UserId.eq(user_id))
            .filter(user_tag::Column::TagId.eq(tag.id))
            .one(&txn)
            .await? {
                Some(user_tag) => user_tag,
                None => {
                    error!("User with id {} and tag with id {} not found", user_id, tag.id);
                    txn.rollback().await?;  // Rollback transaction
                    return Err(DbErr::Custom(format!("User with id {} and tag with id {} not found", user_id, tag.id)));
                }
            };
        // checl if tag have no associate with note_tag
        let note_tag = entities::note_tag::Entity::find()
            .filter(entities::note_tag::Column::TagId.eq(tag.id))
            .one(&txn)
            .await;
        match note_tag {
            Ok(model) => match model {
                Some(_data) => {
                    error!("Tag with id {} is associated with note tag", tag.id);
                    txn.rollback().await?;  // Rollback transaction
                    return Err(DbErr::Custom(format!("Tag with id {} is associated with note tag", tag.id)));
                },
                None => {
                    info!("Tag with id {} is not associated with note tag", tag.id);
                }
            }
            Err(_) => {
                error!("Error checking if tag with id {} is associated with note tag", tag.id);
                txn.rollback().await?;  // Rollback transaction
                return Err(DbErr::Custom(format!("Error checking if tag with id {} is associated with note tag", tag.id)));
            }
        }

        // delete user_tag
        let result_delete = user_tag.delete(&txn).await;
        match result_delete {
            Ok(_) => {
                info!("User tag deleted successfully");
                txn.commit().await?;
            }
            Err(e) => {
                error!("Error deleting user tag: {}", e);
                txn.rollback().await?;
                return Err(DbErr::Custom(format!("Error deleting user tag: {}", e)));
            }
        }

        Ok(())
        
    
    }
}