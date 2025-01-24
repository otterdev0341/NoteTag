use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};


use crate::configuration::db_config::DBConfig;

pub async fn mysql_connec(config: &DBConfig) -> Result<DatabaseConnection, DbErr> {
    let mut opts = ConnectOptions::new(format!(
        "mysql://{}:{}@{}:{}/{}",
        config.db_username, config.db_password, config.db_host, config.db_port, config.db_database_name,
    ));

    opts
        .max_connections(50)
        // .sqlx_logging(true)
        // .sqlx_logging_level(log::LevelFilter::Debug)
        ;
        

    Database::connect(opts).await
}