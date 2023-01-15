use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::env;

async fn connect() -> Result<DatabaseConnection, &str> {
    match env::var("VRTY_DB_CONN_STRING") {
        Ok(conn) => {
            let mut opt = ConnectOptions::new(conn);
            let connection = Database::connect(opt).await;
            match connection {
                Ok(c) => Ok(c),
                Err(_) => Err("Failed connection to db"),
            }
        }
        Err(e) => Err(()),
    }
}
