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
}