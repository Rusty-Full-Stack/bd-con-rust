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
                    .col(ColumnDef::new(Producto::Nombre).string_len(150).not_null())
                    .col(ColumnDef::new(Producto::PrecioUnitario).decimal_len(3, 1))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Producto::Table).to_owned())
            .await
    }
}
