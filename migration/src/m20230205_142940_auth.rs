use crate::Uuid::Uuid;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Auth::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Auth::UserId)
                            .custom(Uuid)
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Auth::Username).string().not_null())
                    .col(ColumnDef::new(Auth::PhcString).text())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Auth::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Auth {
    Table,
    UserId,
    Username,
    PhcString,
}
