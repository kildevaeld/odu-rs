pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20230318_004047_resource_type;
mod m20230318_004236_resource;
mod m20230318_004811_resource_type_types;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20230318_004047_resource_type::Migration),
            Box::new(m20230318_004236_resource::Migration),
            Box::new(m20230318_004811_resource_type_types::Migration),
        ]
    }
}
