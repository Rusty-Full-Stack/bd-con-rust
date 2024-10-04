pub use sea_orm_migration::prelude::*;

mod m20241003_225513_creacion_productos;
mod m20241003_231301_modificar_precision_precio_unitario;
mod m20241003_233551_crear_factura;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241003_225513_creacion_productos::Migration),
            Box::new(m20241003_231301_modificar_precision_precio_unitario::Migration),
            Box::new(m20241003_233551_crear_factura::Migration),
        ]
    }
}
