use std::sync::Arc;

use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, IntoActiveModel, ModelTrait, QueryFilter, Related, Set, TransactionTrait};
use sea_orm_migration::async_trait;
use sqlx::types::chrono::Utc;
use tracing::{error, info};

use crate::domain::{entities::{self, tag, user, user_tag}, repositories::trait_user_x_tag_repository::UserTagRepository};

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
        // Begin the transaction
        let txn = self.db.begin().await?;
    
        // Step 1: Validate the user exists
        if user::Entity::find_by_id(user_id).one(&txn).await?.is_none() {
            return Err(DbErr::Custom(format!("The user with ID {} does not exist", user_id)));
        }
    
        // Step 2: Find the old tag ID
        let old_tag_id = tag::Entity::find()
            .filter(tag::Column::TagName.eq(old_tag))
            .one(&txn)
            .await?
            .ok_or_else(|| DbErr::Custom(format!("The tag '{}' does not exist", old_tag)))?
            .id;
    
        // Step 3: Check if the user-tag association exists
        let user_tag = user_tag::Entity::find()
            .filter(user_tag::Column::UserId.eq(user_id))
            .filter(user_tag::Column::TagId.eq(old_tag_id))
            .one(&txn)
            .await?
            .ok_or_else(|| DbErr::Custom("UserTag record not found".to_string()))?;
    
        // Step 4: Find or create the new tag
        let new_tag_id = match tag::Entity::find()
            .filter(tag::Column::TagName.eq(new_tag))
            .one(&txn)
            .await?
        {
            Some(tag) => tag.id,
            None => {
                let new_tag = tag::ActiveModel {
                    tag_name: Set(new_tag.to_string()),
                    create_at: Set(Some(Utc::now())),
                    updated_at: Set(Some(Utc::now())),
                    ..Default::default()
                };
                new_tag.insert(&txn).await?.id
            }
        };
    
        // Step 5: Update the user-tag association
        let mut user_tag_active: user_tag::ActiveModel = user_tag.into();
        user_tag_active.tag_id = Set(new_tag_id);
        user_tag_active.update(&txn).await?;
    
        // Commit the transaction
        txn.commit().await?;
        Ok(())
    }
    

    async fn delete_tag_from_user(&self, user_id: i32, tag_name: &str) -> Result<(), DbErr> {
        todo!()
    
    }
}