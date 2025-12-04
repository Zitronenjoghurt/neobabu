use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(InventoryItem::Table)
                    .if_not_exists()
                    .col(uuid(InventoryItem::Id).primary_key())
                    .col(string(InventoryItem::UserId))
                    .col(integer(InventoryItem::Kind))
                    .col(json_binary(InventoryItem::State))
                    .col(timestamp(InventoryItem::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp(InventoryItem::UpdatedAt).default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .from(InventoryItem::Table, InventoryItem::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(InventoryItem::Table)
                    .col(InventoryItem::UserId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(InventoryItem::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum InventoryItem {
    Table,
    Id,
    UserId,
    Kind,
    State,
    CreatedAt,
    UpdatedAt,
}
