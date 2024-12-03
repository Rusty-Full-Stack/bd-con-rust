mod entities;
use entities::prelude::*;
use entities::*;

use sea_orm::prelude::*;
use sea_orm::Condition;
use sea_orm::QueryTrait;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conexion = bd_con_rust::obtener_conexion().await?;

    let resultado: Vec<producto::Model> = Producto::find()
        .filter(
            Condition::any()
                .add(producto::Column::PrecioUnitario.gte(100.0))
                .add(producto::Column::Nombre.starts_with("Cafe")),
        )
        .all(&conexion)
        .await?;

    for mi_producto in resultado {
        println!("Id: {}", mi_producto.id);
        println!("Nombre: {}", mi_producto.nombre);
        println!("Precio Unitario: {}", mi_producto.precio_unitario);
        println!("=======================================");
    }

    let query = Producto::find()
        .filter(
            Condition::any()
                .add(producto::Column::PrecioUnitario.gte(100.0))
                .add(producto::Column::Nombre.starts_with("Cafe")),
        )
        .build(sea_orm::DatabaseBackend::Postgres)
        .to_string();

    println!("El query es: {}", query);

    Ok(())
}
