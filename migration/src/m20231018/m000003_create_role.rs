use sea_orm_migration::prelude::*;

use super::entities::Role;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {

    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager.create_table(
            Table::create()
                .table(Role::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Role::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key()
                )
                .col(ColumnDef::new(Role::Name).string().not_null().unique_key())
                .col(ColumnDef::new(Role::Desc).string())
                .to_owned()
        ).await

    }

    async fn down(&self, manager: &SchemaManager) -> Result<(),DbErr> {
        
        manager.drop_table(Table::drop().table(Role::Table).to_owned()).await

    }
    
}