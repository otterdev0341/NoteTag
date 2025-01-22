use note_tag_project::{configuration::db_config::DBConfig, infrastructure::{faring::cors::CORS, http::controller::init_controller_setup::init_controller_setup, mysql::{migrator::Migrator, mysql_connect::mysql_connec}}};
use sea_orm_migration::MigratorTrait;


#[macro_use] extern crate rocket;



#[launch]
async fn rocket() -> _ {
    
    dotenv::dotenv().ok();
    
    let database_config = DBConfig::default();
    let db_connection = mysql_connec(&database_config).await.unwrap();
    Migrator::fresh(&db_connection).await.unwrap();

    rocket::build()
        .attach(CORS)
        .attach(init_controller_setup())
        
        
        
}