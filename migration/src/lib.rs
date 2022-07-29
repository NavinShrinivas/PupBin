pub use sea_orm_migration::prelude::*;

mod m20220729_130509_paste;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220729_130509_paste::Migration),
        ]
    }
}
