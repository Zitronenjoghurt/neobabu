use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(BlackJackUser::Table)
                    .if_not_exists()
                    .col(string(BlackJackUser::UserId).primary_key())
                    .col(big_integer(BlackJackUser::FinalHitScoreTotal).default(0))
                    .col(integer(BlackJackUser::TimesFinalHit).default(0))
                    .col(big_integer(BlackJackUser::FinalStandScoreTotal).default(0))
                    .col(integer(BlackJackUser::TimesFinalStand).default(0))
                    .col(big_integer(BlackJackUser::BustScoreTotal).default(0))
                    .col(integer(BlackJackUser::TimesBusted).default(0))
                    .col(integer(BlackJackUser::BlackjackCount).default(0))
                    .col(integer(BlackJackUser::Wins).default(0))
                    .col(integer(BlackJackUser::Losses).default(0))
                    .col(integer(BlackJackUser::Draws).default(0))
                    .col(integer(BlackJackUser::WinStreak).default(0))
                    .col(integer(BlackJackUser::LongestWinStreak).default(0))
                    .col(integer(BlackJackUser::LossStreak).default(0))
                    .col(integer(BlackJackUser::LongestLossStreak).default(0))
                    .col(integer(BlackJackUser::DrawStreak).default(0))
                    .col(integer(BlackJackUser::LongestDrawStreak).default(0))
                    .col(timestamp(BlackJackUser::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp(BlackJackUser::UpdatedAt).default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .from(BlackJackUser::Table, BlackJackUser::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(BlackJackRivals::Table)
                    .if_not_exists()
                    .col(string(BlackJackRivals::UserId1))
                    .col(string(BlackJackRivals::UserId2))
                    .col(integer(BlackJackRivals::Wins1).default(0))
                    .col(integer(BlackJackRivals::Wins2).default(0))
                    .col(integer(BlackJackRivals::Draws).default(0))
                    .col(integer(BlackJackRivals::LossesBoth).default(0))
                    .col(timestamp(BlackJackRivals::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp(BlackJackRivals::UpdatedAt).default(Expr::current_timestamp()))
                    .primary_key(
                        Index::create()
                            .col(BlackJackRivals::UserId1)
                            .col(BlackJackRivals::UserId2),
                    )
                    .check(
                        Expr::col(BlackJackRivals::UserId1).lt(Expr::col(BlackJackRivals::UserId2)),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(BlackJackRivals::Table, BlackJackRivals::UserId1)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(BlackJackRivals::Table, BlackJackRivals::UserId2)
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
            .drop_table(Table::drop().table(BlackJackUser::Table).to_owned())
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
enum BlackJackUser {
    Table,
    UserId,
    FinalHitScoreTotal,
    TimesFinalHit,
    FinalStandScoreTotal,
    TimesFinalStand,
    BustScoreTotal,
    TimesBusted,
    BlackjackCount,
    Wins,
    Losses,
    Draws,
    WinStreak,
    LongestWinStreak,
    LossStreak,
    LongestLossStreak,
    DrawStreak,
    LongestDrawStreak,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum BlackJackRivals {
    Table,
    UserId1,
    UserId2,
    Wins1,
    Wins2,
    Draws,
    LossesBoth,
    CreatedAt,
    UpdatedAt,
}
