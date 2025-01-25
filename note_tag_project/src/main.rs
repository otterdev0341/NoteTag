use std::sync::Arc;


use note_tag_project::{application::usecase::init_usecase_setup::init_usecase_setup, configuration::{api_doc_config::ApiDoc, db_config::DBConfig, jwt_config}, infrastructure::{faring::cors::CORS, http::controller::init_controller_setup::init_controller_setup, mysql::{migrator::Migrator, mysql_connect::mysql_connec}}};
use sea_orm_migration::MigratorTrait;
use tracing_subscriber;

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;






#[rocket::main]
async fn main() -> Result<(), rocket::Error>  {
    
    dotenv::dotenv().ok();
    
    // Initialize tracing subscriber
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let database_config = DBConfig::default();
    let db_connection = mysql_connec(&database_config).await.unwrap();
    
    
    // Migrator::fresh(&db_connection).await.unwrap();
    Migrator::up(&db_connection, None).await.unwrap();
    // fresh to drop all table and try migrate all new table
    
    // to config * at rocket use custom(conifg) instend of build()
    // let config = Config::figment()
    //     .merge(("address", "127.0.0.1"))
    //     .merge(("port", 8000));

    let db_arc = Arc::new(db_connection);

    rocket::build()
        .attach(CORS)
        // .manage(init_usecase_setup(db_connection.clone())) // Attach use case setup
        .attach(init_usecase_setup(Arc::clone(&db_arc)))
        .attach(init_controller_setup())
        .manage(jwt_config::JwtSecret::default())
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui/<_..>")
                .url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
        .launch()
        .await?;
        
        
    Ok(())
}


