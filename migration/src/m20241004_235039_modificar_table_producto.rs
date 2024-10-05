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
                            .decimal_len(10, 2)
                            .not_null(),
                    )
                    .add_column(ColumnDef::new(Producto::Descripcion).text())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Producto::Table)
                    .drop_column(Producto::Descripcion)
                    .modify_column(
                        ColumnDef::new(Producto::PrecioUnitario)
                            .decimal_len(3, 1)
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }
}
