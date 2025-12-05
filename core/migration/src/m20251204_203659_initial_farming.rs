use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(FarmingWorld::Table)
                    .if_not_exists()
                    .col(string(FarmingWorld::UserId))
                    .col(integer(FarmingWorld::Index))
                    .col(json_binary(FarmingWorld::Data))
                    .col(timestamp(FarmingWorld::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp(FarmingWorld::UpdatedAt).default(Expr::current_timestamp()))
                    .primary_key(
                        Index::create()
                            .col(FarmingWorld::UserId)
                            .col(FarmingWorld::Index),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(FarmingWorld::Table, FarmingWorld::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(FarmingWorld::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum FarmingWorld {
    Table,
    UserId,
    Index,
    Data,
    CreatedAt,
    UpdatedAt,
}
