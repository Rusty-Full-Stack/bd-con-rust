mod entities;
use entities::prelude::*;
use entities::*;

use sea_orm::prelude::*;
use sea_orm::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conexion = bd_con_rust::obtener_conexion().await?;

    let resultado: Option<producto::Model> = Producto::find_by_id(2).one(&conexion).await?;

    if let Some(producto_encontrado) = resultado {
        println!("producto encontrado es: ");
        println!("id: {}", producto_encontrado.id);
        println!("nombre: {}", producto_encontrado.nombre);
    } else {
        println!("producto no encontrado");
    };

    Ok(())
}
