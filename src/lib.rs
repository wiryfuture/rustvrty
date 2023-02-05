mod db;
mod sec;
use crate::db::connect;
pub use crate::sec::{login, AuthType, User};
use migration::{Migrator, MigratorTrait};

pub async fn migration() {
    let connection = connect().await.unwrap();
    Migrator::up(&connection, None).await.unwrap();
}
