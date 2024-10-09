mod entities;
use entities::prelude::*;
use entities::*;

use sea_orm::prelude::*;
use sea_orm::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conexion = bd_con_rust::obtener_conexion().await?;

    let nuevo_producto = producto::ActiveModel {
        nombre: ActiveValue::Set("Nuevo producto".to_owned()),
        precio_unitario: ActiveValue::Set(Decimal::from_f64_retain(100.50).unwrap()),
        ..Default::default()
    };

    let resultado_insert_producto = Producto::insert(nuevo_producto).exec(&conexion).await?;

    let nueva_factura = factura::ActiveModel {
        nombre_cliente: ActiveValue::Set(Some("Rusty Full Stack".to_owned())),
        total: ActiveValue::Set(Some(Decimal::from_f64_retain(100.50).unwrap())),
        ..Default::default()
    };

    let resultado_insert_factura = Factura::insert(nueva_factura).exec(&conexion).await?;

    let detalle = detalle_factura::ActiveModel {
        producto_id: ActiveValue::Set(resultado_insert_producto.last_insert_id),
        factura_id: ActiveValue::Set(resultado_insert_factura.last_insert_id),
        cantidad: ActiveValue::Set(1),
        subtotal: ActiveValue::Set(Some(Decimal::from_f64_retain(100.50).unwrap())),
    };

    DetalleFactura::insert_many(vec![detalle])
        .exec(&conexion)
        .await?;

    println!("Registros exitosos");

    Ok(())
}
