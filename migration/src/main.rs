use sea_orm_migration::prelude::*;

#[async_std::main]
async fn main() {
    dotenv::dotenv().expect("dotenv");
    cli::run_cli(migration::Migrator).await;
}
