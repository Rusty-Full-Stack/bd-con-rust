use sea_orm_migration::{prelude::*, schema::*};

#[derive(Iden)]
pub enum Producto {
    Table,
    PrecioUnitario,
    Descripcion,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Producto::Table)
                    .modify_column(
                        ColumnDef::new(Producto::PrecioUnitario)
                            .decimal_len(7, 2)
                            .not_null(),
                    )
                    .add_column(ColumnDef::new(Producto::Descripcion).string())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Producto::Table)
                    .drop_column(Producto::Descripcion) // Eliminar la columna agregada
                    .modify_column(
                        ColumnDef::new(Producto::PrecioUnitario)
                            .decimal_len(5, 2) // Revertir la longitud original (si es necesario)
                            .not_null(), // Si era NOT NULL antes
                    )
                    .to_owned(),
            )
            .await
    }
}
