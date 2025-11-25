use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(YoutubeChannel::Table)
                    .if_not_exists()
                    .col(string(YoutubeChannel::Id).primary_key())
                    .col(string(YoutubeChannel::Name))
                    .col(string_null(YoutubeChannel::IconUrl).default(Expr::null()))
                    .col(timestamp(YoutubeChannel::NextResubscriptionAt))
                    .col(timestamp(YoutubeChannel::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp(YoutubeChannel::UpdatedAt).default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(YoutubeVideo::Table)
                    .if_not_exists()
                    .col(string(YoutubeVideo::Id).primary_key())
                    .col(string(YoutubeVideo::ChannelId))
                    .col(string(YoutubeVideo::Title))
                    .col(string_null(YoutubeVideo::ThumbnailUrl).default(Expr::null()))
                    .col(boolean(YoutubeVideo::NotificationSent).default(false))
                    .col(timestamp(YoutubeVideo::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp(YoutubeVideo::UpdatedAt).default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .from(YoutubeVideo::Table, YoutubeVideo::ChannelId)
                            .to(YoutubeChannel::Table, YoutubeChannel::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(GuildYoutube::Table)
                    .if_not_exists()
                    .col(string(GuildYoutube::GuildId).primary_key())
                    .col(boolean(GuildYoutube::Enabled).default(false))
                    .col(string_null(GuildYoutube::NotificationChannelId).default(Expr::null()))
                    .col(string_null(GuildYoutube::NotificationRoleId).default(Expr::null()))
                    .col(timestamp(GuildYoutube::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp(GuildYoutube::UpdatedAt).default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .from(GuildYoutube::Table, GuildYoutube::GuildId)
                            .to(Guild::Table, Guild::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(GuildYoutubeChannel::Table)
                    .if_not_exists()
                    .col(string(GuildYoutubeChannel::GuildId))
                    .col(string(GuildYoutubeChannel::ChannelId))
                    .col(
                        timestamp(GuildYoutubeChannel::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .primary_key(
                        Index::create()
                            .col(GuildYoutubeChannel::GuildId)
                            .col(GuildYoutubeChannel::ChannelId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(GuildYoutubeChannel::Table, GuildYoutubeChannel::GuildId)
                            .to(Guild::Table, Guild::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(GuildYoutubeChannel::Table, GuildYoutubeChannel::ChannelId)
                            .to(YoutubeChannel::Table, YoutubeChannel::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(YoutubeChannel::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(YoutubeVideo::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(GuildYoutube::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(GuildYoutubeChannel::Table).to_owned())
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
enum YoutubeChannel {
    Table,
    Id,
    Name,
    IconUrl,
    NextResubscriptionAt,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum YoutubeVideo {
    Table,
    Id,
    ChannelId,
    Title,
    ThumbnailUrl,
    NotificationSent,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum GuildYoutube {
    Table,
    GuildId,
    Enabled,
    NotificationChannelId,
    NotificationRoleId,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum GuildYoutubeChannel {
    Table,
    GuildId,
    ChannelId,
    CreatedAt,
}
