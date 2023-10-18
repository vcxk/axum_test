pub use sea_orm_migration::prelude::*;

mod m20231018;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20231018::m000001_create_user::Migration),
            Box::new(m20231018::m000002_create_token::Migration),
            Box::new(m20231018::m000003_create_role::Migration),
            Box::new(m20231018::m000004_create_user_role::Migration),
        ]
    }
}
