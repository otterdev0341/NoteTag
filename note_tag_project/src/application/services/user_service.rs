use crate::infrastructure::mysql::repositories::impl_user_repository::ImplUserRepository;

pub struct UserService {
    user_repository: ImplUserRepository
}

impl UserService {
    pub fn new(user_repository: ImplUserRepository) -> Self {
        UserService {
            user_repository
        }
    }
    pub fn create_user(&self) {
        todo!()
    }
    pub fn get_user_by_id(&self) {
        todo!()
    }
    pub fn get_all_user(&self) {
        todo!()
    }
    pub fn update_user_by_id(&self) {
        todo!()
    }
    pub fn delete_user_by_id(&self) {
        todo!()
    }
}