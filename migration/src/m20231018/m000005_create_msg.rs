use sea_orm_migration::prelude::*;

use crate::entities::MqttMessage as Model;


#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager.create_table(
            Table::create()
                .table(Model::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Model::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(Model::Sn).string().not_null())
                .col(ColumnDef::new(Model::Msg).string().not_null())
                .col(ColumnDef::new(Model::Type).string())
                .col(ColumnDef::new(Model::CreateTime).date_time())
                .to_owned(),
            ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Model::Table).to_owned())
            .await
    }
}
