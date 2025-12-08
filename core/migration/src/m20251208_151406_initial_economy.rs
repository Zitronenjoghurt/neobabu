use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Economy::Table)
                    .if_not_exists()
                    .col(string(Economy::UserId))
                    .col(small_integer(Economy::Currency))
                    .col(big_integer(Economy::Amount))
                    .primary_key(Index::create().col(Economy::UserId).col(Economy::Currency))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Economy::Table, Economy::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(EconomyPending::Table)
                    .if_not_exists()
                    .col(string(EconomyPending::ReferenceId))
                    .col(string(EconomyPending::UserId))
                    .col(small_integer(EconomyPending::Currency))
                    .col(big_integer(EconomyPending::Amount))
                    .col(timestamp(EconomyPending::ExpiresAt))
                    .col(timestamp(EconomyPending::CreatedAt).default(Expr::current_timestamp()))
                    .primary_key(
                        Index::create()
                            .col(EconomyPending::ReferenceId)
                            .col(EconomyPending::UserId)
                            .col(EconomyPending::Currency),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(EconomyPending::Table, EconomyPending::UserId)
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
            .drop_table(Table::drop().table(Economy::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(EconomyPending::Table).to_owned())
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
enum Economy {
    Table,
    UserId,
    Currency,
    Amount,
}

#[derive(DeriveIden)]
enum EconomyPending {
    Table,
    ReferenceId,
    UserId,
    Currency,
    Amount,
    ExpiresAt,
    CreatedAt,
}
