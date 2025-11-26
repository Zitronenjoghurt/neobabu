use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(string(User::Id).primary_key())
                    .col(string_null(User::Username).default(Expr::null()))
                    .col(string_null(User::AvatarHash).default(Expr::null()))
                    .col(string_null(User::EncryptedOauthToken).default(Expr::null()))
                    .col(big_integer(User::Permissions).default(0_u64 as i64))
                    .col(timestamp(User::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp(User::UpdatedAt).default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Guild::Table)
                    .if_not_exists()
                    .col(string(Guild::Id).primary_key())
                    .col(timestamp(Guild::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp(Guild::UpdatedAt).default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UserGuild::Table)
                    .if_not_exists()
                    .col(string(UserGuild::UserId))
                    .col(string(UserGuild::GuildId))
                    .col(timestamp(UserGuild::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp(UserGuild::UpdatedAt).default(Expr::current_timestamp()))
                    .primary_key(
                        Index::create()
                            .col(UserGuild::UserId)
                            .col(UserGuild::GuildId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(UserGuild::Table, UserGuild::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(UserGuild::Table, UserGuild::GuildId)
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
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Guild::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(UserGuild::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Username,
    AvatarHash,
    EncryptedOauthToken,
    Permissions,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Guild {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum UserGuild {
    Table,
    UserId,
    GuildId,
    CreatedAt,
    UpdatedAt,
}
