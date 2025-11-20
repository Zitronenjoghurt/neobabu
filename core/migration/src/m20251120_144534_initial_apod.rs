use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Apod::Table)
                    .if_not_exists()
                    .col(small_integer(Apod::Day))
                    .col(small_integer(Apod::Month))
                    .col(small_integer(Apod::Year))
                    .col(string_null(Apod::Title).default(Expr::null()))
                    .col(string_null(Apod::Explanation).default(Expr::null()))
                    .col(string_null(Apod::Url).default(Expr::null()))
                    .col(string_null(Apod::HdUrl).default(Expr::null()))
                    .col(string_null(Apod::ThumbnailUrl).default(Expr::null()))
                    .col(string_null(Apod::MediaType).default(Expr::null()))
                    .col(string_null(Apod::Copyright).default(Expr::null()))
                    .col(boolean(Apod::WasAnnounced).default(false))
                    .primary_key(
                        Index::create()
                            .col(Apod::Day)
                            .col(Apod::Month)
                            .col(Apod::Year),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(GuildApod::Table)
                    .col(string(GuildApod::GuildId).primary_key())
                    .col(boolean(GuildApod::Enabled).default(false))
                    .col(string_null(GuildApod::NotificationChannelId).default(Expr::null()))
                    .col(string_null(GuildApod::NotificationRoleId).default(Expr::null()))
                    .col(timestamp(GuildApod::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp(GuildApod::UpdatedAt).default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .from(GuildApod::Table, GuildApod::GuildId)
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
            .drop_table(Table::drop().table(Apod::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(GuildApod::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Guild {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Apod {
    Table,
    Day,
    Month,
    Year,
    Title,
    Explanation,
    Url,
    HdUrl,
    ThumbnailUrl,
    MediaType,
    Copyright,
    WasAnnounced,
}

#[derive(DeriveIden)]
enum GuildApod {
    Table,
    GuildId,
    Enabled,
    NotificationChannelId,
    NotificationRoleId,
    CreatedAt,
    UpdatedAt,
}
