use sea_orm_migration::prelude::*;

use super::entities::UserRole;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self,manager: &SchemaManager) -> Result<(),DbErr> {
        manager.create_table(
            Table::create()
                .table(UserRole::Table)
                .if_not_exists()
                .col(ColumnDef::new(UserRole::Id).integer().not_null().auto_increment().primary_key())
                .col(ColumnDef::new(UserRole::RoleId).integer().not_null())
                .col(ColumnDef::new(UserRole::UserId).integer().not_null())
                .col(ColumnDef::new(UserRole::CreateTime).date_time().not_null())
                .col(ColumnDef::new(UserRole::CreatorId).integer().not_null())
                .to_owned()
        ).await
    }

    async fn down(&self,manager:&SchemaManager) -> Result<(),DbErr> {
        manager.drop_table(Table::drop().table(UserRole::Table).to_owned()).await
    }
}