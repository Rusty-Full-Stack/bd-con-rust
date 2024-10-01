use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbErr, Statement};

const DATABASE_URL: &str = "postgres://postgres:postgres@localhost:5432";
const DB_NAME: &str = "curso";
const CREATE_DATABASE: bool = false;

pub async fn obtener_conexion() -> Result<DatabaseConnection, DbErr> {
    if CREATE_DATABASE {
        let conexion = Database::connect(DATABASE_URL).await?;

        let query = format!("CREATE DATABASE {}", DB_NAME);

        let stmt = Statement::from_string(conexion.get_database_backend(), query);

        conexion.execute(stmt).await?;
    }

    let connection_string = format!("{}/{}", DATABASE_URL, DB_NAME);

    let conexion = Database::connect(connection_string).await?;
    Ok(conexion)
}
