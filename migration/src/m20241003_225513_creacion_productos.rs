use sea_orm_migration::{prelude::*, schema::*};

#[derive(Iden)]
pub enum Producto {
    Table,
    Id,
    Nombre,
    PrecioUnitario,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Producto::Table)
                    .col(
                        ColumnDef::new(Producto::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Producto::Nombre)
                            .string()
                            .not_null()
                            .string_len(150),
                    )
                    .col(
                        ColumnDef::new(Producto::PrecioUnitario)
                            .double()
                            .not_null()
                            .decimal_len(2, 7),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Producto::Table).to_owned())
            .await
    }
}
