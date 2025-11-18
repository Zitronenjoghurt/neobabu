use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .if_not_exists()
                    .table(RPSUser::Table)
                    .col(string(RPSUser::UserId).primary_key())
                    .col(integer(RPSUser::TimesRock).default(0))
                    .col(integer(RPSUser::TimesPaper).default(0))
                    .col(integer(RPSUser::TimesScissors).default(0))
                    .col(timestamp(RPSUser::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp(RPSUser::UpdatedAt).default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .from(RPSUser::Table, RPSUser::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .if_not_exists()
                    .table(RPSGames::Table)
                    .col(string(RPSGames::UserId1))
                    .col(string(RPSGames::UserId2))
                    .col(integer(RPSGames::Wins1).default(0))
                    .col(integer(RPSGames::Wins2).default(0))
                    .col(integer(RPSGames::Draws).default(0))
                    .col(timestamp(RPSGames::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp(RPSGames::UpdatedAt).default(Expr::current_timestamp()))
                    .primary_key(
                        Index::create()
                            .col(RPSGames::UserId1)
                            .col(RPSGames::UserId2),
                    )
                    .check(Expr::col(RPSGames::UserId1).lt(Expr::col(RPSGames::UserId2)))
                    .foreign_key(
                        ForeignKey::create()
                            .from(RPSGames::Table, RPSGames::UserId1)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(RPSGames::Table, RPSGames::UserId2)
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
            .drop_table(Table::drop().table(RPSGames::Table).to_owned())
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
enum RPSUser {
    Table,
    UserId,
    TimesRock,
    TimesPaper,
    TimesScissors,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum RPSGames {
    Table,
    UserId1,
    UserId2,
    Wins1,
    Wins2,
    Draws,
    CreatedAt,
    UpdatedAt,
}
