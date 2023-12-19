use sea_orm_migration::prelude::*;
use crate::entities::MqttAht20Statistics as Model;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self,manager:&SchemaManager) -> Result<(),DbErr> {
        manager.create_table(
            Table::create()
                .table(Model::Table)
                .col(
                    ColumnDef::new(Model::Id)
                    .integer()
                    .not_null()
                    .primary_key()
                    .auto_increment()
                )
                .col(ColumnDef::new(Model::Sn).string_len(64).not_null())
                .col(ColumnDef::new(Model::HumiData).binary())
                .col(ColumnDef::new(Model::HumiMax).double())
                .col(ColumnDef::new(Model::HumiMin).double())
                .col(ColumnDef::new(Model::TempData).binary())
                .col(ColumnDef::new(Model::TempMax).double())
                .col(ColumnDef::new(Model::TempMin).double())
                .col(ColumnDef::new(Model::StartTime).date_time().not_null())
                .col(ColumnDef::new(Model::Duration).unsigned().not_null())
                .to_owned()
        ).await
    }

    async fn down(&self,manager:&SchemaManager) -> Result<(),DbErr> {
        manager.drop_table(Table::drop().table(Model::Table).to_owned()).await
    }
}