pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_house;
mod m20240115_160419_create_room;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_house::Migration),
            Box::new(m20240115_160419_create_room::Migration),
        ]
    }
}
