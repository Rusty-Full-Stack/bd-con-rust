mod entities;
use entities::prelude::*;
use entities::*;

use sea_orm::prelude::*;
use sea_orm::Condition;
use sea_orm::QueryTrait;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conexion = bd_con_rust::obtener_conexion().await?;

    let productos_y_detalles: Vec<(producto::Model, Option<detalle_factura::Model>)> =
        Producto::find_by_id(1)
            .find_also_related(DetalleFactura)
            .all(&conexion)
            .await?;

    println!("productos y detalles: {:?}", productos_y_detalles);
    println!("======================================");
    let productos_con_detalles: Vec<(producto::Model, Vec<detalle_factura::Model>)> =
        Producto::find_by_id(1)
            .find_with_related(DetalleFactura)
            .all(&conexion)
            .await?;

    println!("productos con detalles: {:?}", productos_con_detalles);

    Ok(())
}
