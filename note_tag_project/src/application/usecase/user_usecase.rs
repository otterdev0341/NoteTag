use crate::application::services::user_service::UserService;

pub struct UserUseCase {
    user_service: UserService
}

impl UserUseCase {
    pub fn new(user_service: UserService) -> Self {
        UserUseCase {
            user_service
        }
    }
    pub fn sign_up(&self) {
        todo!()
    }
    pub fn sign_in(&self) {
        todo!()
    }

    pub fn delete_account(&self) {
        todo!()
    }

    pub fn update_account(&self) {
        todo!()
    }

}