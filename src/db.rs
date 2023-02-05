use dotenv::dotenv;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::env;

pub async fn connect() -> Result<DatabaseConnection, &'static str> {
    dotenv().ok();
    match env::var("VRTY_DB_CONN_STRING") {
        Ok(connection_string) => {
            let opt = ConnectOptions::new(connection_string);
            let connection = Database::connect(opt).await;
            match connection {
                Ok(c) => Ok(c),
                Err(_) => Err("Failed connection to db"),
            }
        }
        Err(e) => {
            eprintln! {"{e}"};
            Err("No db connection string found - add it to envvar VRTY_DB_CONN_STRING")
        }
    }
}
