use note_tag_project::{configuration::db_config::DBConfig, infrastructure::{faring::cors::CORS, mysql::{migrator::Migrator, mysql_connect::mysql_connec}}};
use sea_orm_migration::MigratorTrait;


#[macro_use] extern crate rocket;

#[get("/")]
async fn index() -> &'static str {
    "Hello, world!"
}


#[launch]
async fn rocket() -> _ {
    
    dotenv::dotenv().ok();
    
    let database_config = DBConfig::default();
    let db_connection = mysql_connec(&database_config).await.unwrap();
    Migrator::up(&db_connection, None).await.unwrap();

    rocket::build()
        .attach(CORS)
        .mount("/", routes![index])
}