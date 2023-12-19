use sea_orm_migration::prelude::*;
use crate::entities::MqttAht20 as Model;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self,manager:&SchemaManager,) -> Result<(),DbErr> {
        manager.create_table(
            Table::create()
                .table(Model::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Model::Id)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key()
                )
                .col(ColumnDef::new(Model::Sn).string_len(64).not_null())
                .col(ColumnDef::new(Model::Humidity).double().not_null())
                .col(ColumnDef::new(Model::Temperature).double().not_null())
                .col(ColumnDef::new(Model::CreateTime).date_time().not_null())
                .to_owned()
        ).await
    }
    async fn down(&self,manager:&SchemaManager) -> Result<(),DbErr> {
        manager.drop_table(Table::drop().table(Model::Table).to_owned()).await
    }
}
