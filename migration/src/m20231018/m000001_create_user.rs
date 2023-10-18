use sea_orm_migration::prelude::*;

use super::entities::User;


#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager.create_table(
            Table::create()
                .table(User::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(User::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(User::Account).string().unique_key().not_null())
                .col(ColumnDef::new(User::PassHash).string().not_null())
                .col(ColumnDef::new(User::Email).string())
                .col(ColumnDef::new(User::Phone).string())
                .col(ColumnDef::new(User::Sex).string())
                .col(ColumnDef::new(User::BirthDay).string())
                .col(ColumnDef::new(User::CreateTime).date_time())
                .to_owned(),
            ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}
