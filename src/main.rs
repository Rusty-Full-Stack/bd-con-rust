mod entities;
use entities::prelude::*;
use entities::*;

use sea_orm::prelude::*;
use sea_orm::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conexion = bd_con_rust::obtener_conexion().await?;

    let detalle_factura_borrar = detalle_factura::ActiveModel {
        factura_id: ActiveValue::Set(1),
        producto_id: ActiveValue::Set(1),
        ..Default::default()
    };

    detalle_factura_borrar.delete(&conexion).await?;

    println!("Borrado exitoso!");

    Ok(())
}
