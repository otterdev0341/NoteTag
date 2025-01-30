use std::sync::Arc;

use rocket::fairing::AdHoc;
use sea_orm::DatabaseConnection;

use crate::infrastructure::mysql::repositories::impl_note_repository::ImplNoteRepository;
use crate::infrastructure::mysql::repositories::impl_note_x_status_repository::ImplNoteStatusRepository;
use crate::infrastructure::mysql::repositories::impl_user_note_query_repository::ImplUserNoteQueryRepository;
use crate::infrastructure::mysql::repositories::impl_user_repository::ImplUserRepository;
use crate::application::usecase::user_usecase::UserUseCase;
use crate::infrastructure::mysql::repositories::impl_user_x_tag_repository::ImplUserTagRepository;

use super::user_query_usecase::UserQueryUsecase;
use super::{note_status, note_usecase, user_query_usecase};
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

        let note_status_repository = ImplNoteStatusRepository{
            db: Arc::clone(&db_connection)
        };
        let note_status_usecase = Arc::new(note_status::NoteStatusUseCase::new(Arc::new(note_status_repository)).await);

        let user_query_prpository = ImplUserNoteQueryRepository{
            db: Arc::clone(&db_connection)
        };

        let user_query_usecase = Arc::new(UserQueryUsecase::new(Arc::new(user_query_prpository)).await);
        
        rocket.manage(Arc::clone(&db_connection))
              .manage(user_usecase)
              .manage(user_tag_usecase)
              .manage(note_usecase)
              .manage(note_status_usecase)
              .manage(user_query_usecase)
    })
}