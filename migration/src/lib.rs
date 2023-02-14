pub use sea_orm_migration::prelude::*;

mod m20230205_142940_auth;
mod m20230214_152908_tokens;

#[derive(Iden)]
pub enum Uuid {
    Uuid,
}

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230205_142940_auth::Migration),
            Box::new(m20230214_152908_tokens::Migration),
        ]
    }
}
