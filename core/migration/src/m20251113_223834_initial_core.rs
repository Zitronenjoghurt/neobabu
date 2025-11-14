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

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Guild::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
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
