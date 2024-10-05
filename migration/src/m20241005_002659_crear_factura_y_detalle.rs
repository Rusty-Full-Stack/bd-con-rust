use super::m20241004_233725_crear_table_product::Producto;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(Iden)]
pub enum Factura {
    Table,
    Id,
    NombreCliente,
    Total,
}

#[derive(Iden)]
pub enum DetalleFactura {
    Table,
    FacturaId,
    ProductoId,
    Cantidad,
    Subtotal,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Factura::Table)
                    .col(
                        ColumnDef::new(Factura::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Factura::NombreCliente).string_len(250))
                    .col(ColumnDef::new(Factura::Total).decimal_len(7, 2))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(DetalleFactura::Table)
                    .col(
                        ColumnDef::new(DetalleFactura::FacturaId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(DetalleFactura::ProductoId)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(DetalleFactura::Subtotal).decimal_len(7, 2))
                    .col(
                        ColumnDef::new(DetalleFactura::Cantidad)
                            .integer()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .col(DetalleFactura::FacturaId)
                            .col(DetalleFactura::ProductoId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-factura")
                            .from(DetalleFactura::Table, DetalleFactura::FacturaId)
                            .to(Factura::Table, Factura::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-producto")
                            .from(DetalleFactura::Table, DetalleFactura::ProductoId)
                            .to(Producto::Table, Producto::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(DetalleFactura::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Factura::Table).to_owned())
            .await
    }
}
