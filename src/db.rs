use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::env;

async fn connect() -> Result<DatabaseConnection, &'static str> {
    match env::var("VRTY_DB_CONN_STRING") {
        Ok(conn) => {
            let opt = ConnectOptions::new(conn);
            let connection = Database::connect(opt).await;
            match connection {
                Ok(c) => Ok(c),
                Err(_) => Err("Failed connection to db"),
            }
        }
        Err(_) => Err("No db connection string found - add it to envvar VRTY_DB_CONN_STRING"),
    }
}
