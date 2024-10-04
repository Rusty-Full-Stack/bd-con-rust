use sea_orm::{ConnectionTrait, Statement};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conexion = bd_con_rust::obtener_conexion().await?;
    println!("conectado!");

    Ok(())
}
