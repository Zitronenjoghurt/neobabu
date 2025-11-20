use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserBirthday::Table)
                    .col(string(UserBirthday::UserId).primary_key())
                    .col(small_integer(UserBirthday::Day))
                    .col(small_integer(UserBirthday::Month))
                    .col(small_integer_null(UserBirthday::Year))
                    .col(timestamp(UserBirthday::NextBirthday))
                    .col(timestamp(UserBirthday::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp(UserBirthday::UpdatedAt).default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .from(UserBirthday::Table, UserBirthday::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(GuildBirthday::Table)
                    .col(string(GuildBirthday::GuildId).primary_key())
                    .col(boolean(GuildBirthday::Enabled).default(false))
                    .col(string_null(GuildBirthday::NotificationChannelId).default(Expr::null()))
                    .col(timestamp(GuildBirthday::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp(GuildBirthday::UpdatedAt).default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .from(GuildBirthday::Table, GuildBirthday::GuildId)
                            .to(Guild::Table, Guild::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserBirthday::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(GuildBirthday::Table).to_owned())
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
enum Guild {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum UserBirthday {
    Table,
    UserId,
    Day,
    Month,
    Year,
    NextBirthday,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum GuildBirthday {
    Table,
    GuildId,
    Enabled,
    NotificationChannelId,
    CreatedAt,
    UpdatedAt,
}
