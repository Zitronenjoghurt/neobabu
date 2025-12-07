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
