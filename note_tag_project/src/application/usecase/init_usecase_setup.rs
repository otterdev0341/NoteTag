use std::sync::Arc;

use rocket::fairing::AdHoc;
use sea_orm::DatabaseConnection;

use crate::infrastructure::mysql::repositories::impl_note_repository::ImplNoteRepository;
use crate::infrastructure::mysql::repositories::impl_user_repository::ImplUserRepository;
use crate::application::usecase::user_usecase::UserUseCase;
use crate::infrastructure::mysql::repositories::impl_user_x_tag_repository::ImplUserTagRepository;

use super::note_usecase;
use super::user_tag_usecase::UserTagUseCase;

pub fn init_usecase_setup(db_connection: Arc<DatabaseConnection>) -> AdHoc {
    AdHoc::on_ignite("Initialize use cases", |rocket| async move {
        

        // Initialize use cases
        let user_repository = ImplUserRepository{
            db: Arc::clone(&db_connection)
        };
        let user_usecase = Arc::new(UserUseCase::new(Arc::new(user_repository)).await);

        let use_tag_repository = ImplUserTagRepository{
            db: Arc::clone(&db_connection)
        };
        let user_tag_usecase = Arc::new(UserTagUseCase::new(Arc::new(use_tag_repository)).await);

        let note_repository = ImplNoteRepository{
            db: Arc::clone(&db_connection)
        };
        let note_usecase = Arc::new(note_usecase::NoteUseCase::new(Arc::new(note_repository)).await);

        rocket.manage(Arc::clone(&db_connection))
              .manage(user_usecase)
              .manage(user_tag_usecase)
              .manage(note_usecase)
    })
}