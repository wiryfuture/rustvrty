use crate::m20230205_142940_auth::Auth;
use crate::Uuid::Uuid;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Tokens::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Tokens::TokenId)
                            .custom(Uuid)
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Tokens::UserId).custom(Uuid).not_null())
                    .col(ColumnDef::new(Tokens::Expiration).date_time().not_null())
                    .col(ColumnDef::new(Tokens::Ip).string())
                    .col(ColumnDef::new(Tokens::UserAgent).text())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_USERID")
                            .from(Tokens::Table, Tokens::UserId)
                            .to(Auth::Table, Auth::UserId)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Tokens::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Tokens {
    Table,
    TokenId,
    UserId,
    Expiration,
    Ip,
    UserAgent,
}
