use sea_orm_migration::prelude::*;

use super::entities::LoginToken;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager.create_table(
            Table::create()
                .table(LoginToken::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(LoginToken::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(LoginToken::Token).string().unique_key().not_null())
                .col(ColumnDef::new(LoginToken::UserId).integer().not_null())
                .col(ColumnDef::new(LoginToken::Time).date_time())
                .col(ColumnDef::new(LoginToken::Ip).string())
                .col(ColumnDef::new(LoginToken::Device).string())
                .to_owned(),
            ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(LoginToken::Table).to_owned())
            .await
    }
}
