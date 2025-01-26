use std::sync::Arc;

use crate::domain::repositories::trait_user_x_tag_repository::UserTagRepository;

pub struct UserTagUseCase<T>
where 
    T: UserTagRepository + Send + Sync,
{
    user_tag_repository: Arc<T>
}


impl<T> UserTagUseCase<T>
where 
    T: UserTagRepository + Send + Sync,
{
    pub async fn new(user_tag_repository: Arc<T>) -> Self {
        Self {
            user_tag_repository: user_tag_repository
        }
    }

    pub async fn create_user_tag(&self, user_id: i32, tag_name: &str) -> Result<(), String> {
        let result = self.user_tag_repository.create_user_tag(user_id, tag_name).await;
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err("Error adding user tag".to_string())
        }
    }

    pub async fn get_user_tags(&self, user_id: i32) -> Result<Vec<String>, String> {
        let result = self.user_tag_repository.get_user_tags(user_id).await;
        match result {
            Ok(tags) => Ok(tags),
            Err(_) => Err("Error getting user tags".to_string())
        }
    }

    pub async fn update_user_tag(&self, user_id: i32, old_tag: &str, new_tag: &str) -> Result<(), String> {
        let result = self.user_tag_repository.update_user_tag(user_id, old_tag, new_tag).await;
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err("Error updating user tag".to_string())
        }
    }
}