use sea_orm::{ConnectionTrait, Statement};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conexion = bd_con_rust::obtener_conexion().await?;
    println!("conectado!");

    let crear_tabla = Statement::from_string(
        conexion.get_database_backend(),
        r#"
        CREATE TABLE prueba(
            id int,
            nombre varchar(150)
        )
        "#,
    );

    conexion.execute(crear_tabla).await?;

    let insert = Statement::from_sql_and_values(
        conexion.get_database_backend(),
        r#"
        INSERT INTO prueba(id, nombre)
        VALUES($1, $2)
        "#,
        [1.into(), "Rusty Full Stack".into()],
    );

    conexion.execute(insert).await?;

    println!("Registro exitoso!");
    Ok(())
}
