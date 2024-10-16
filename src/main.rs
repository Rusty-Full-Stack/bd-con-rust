mod entities;
use entities::prelude::*;
use entities::*;

use sea_orm::prelude::*;
use sea_orm::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conexion = bd_con_rust::obtener_conexion().await?;

    let producto_actualizar = producto::ActiveModel {
        id: ActiveValue::Set(1),
        nombre: ActiveValue::Set("Cafetera".to_owned()),
        descripcion: ActiveValue::Set(Some("Mi maravillosa cafetera!".to_owned())),
        ..Default::default()
    };

    producto_actualizar.update(&conexion).await?;

    println!("Actualizacion exitosa!");

    Ok(())
}
