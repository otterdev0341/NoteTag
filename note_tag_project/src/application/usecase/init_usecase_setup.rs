use std::sync::Arc;

use rocket::fairing::AdHoc;
use sea_orm::DatabaseConnection;

use crate::infrastructure::mysql::repositories::impl_user_repository::ImplUserRepository;
use crate::application::usecase::user_usecase::UserUseCase;

pub fn init_usecase_setup(db_connection: Arc<DatabaseConnection>) -> AdHoc {
    AdHoc::on_ignite("Initialize use cases", |rocket| async move {
        
        let user_repository = ImplUserRepository{
            db: Arc::clone(&db_connection)
        };
        let user_case = Arc::new(UserUseCase::new(Arc::new(user_repository)).await);


        rocket.manage(Arc::clone(&db_connection))
              .manage(user_case)
    })
}