pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20241004_233725_crear_table_product;
mod m20241004_235039_modificar_table_producto;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20241004_233725_crear_table_product::Migration),
            Box::new(m20241004_235039_modificar_table_producto::Migration),
        ]
    }
}
