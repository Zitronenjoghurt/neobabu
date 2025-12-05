use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Farming::Table)
                    .if_not_exists()
                    .col(string(Farming::UserId).primary_key())
                    .col(big_integer(Farming::StoryFlags).default(0))
                    .col(timestamp(Farming::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp(Farming::UpdatedAt).default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Farming::Table, Farming::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(FarmingWorld::Table)
                    .if_not_exists()
                    .col(string(FarmingWorld::UserId))
                    .col(integer(FarmingWorld::Index))
                    .col(string(FarmingWorld::Name))
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
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Farming::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(FarmingWorld::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Farming {
    Table,
    UserId,
    StoryFlags,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum FarmingWorld {
    Table,
    UserId,
    Index,
    Name,
    Data,
    CreatedAt,
    UpdatedAt,
}
