use sea_orm_migration::{prelude::*, sea_query::extension::postgres::Type};

use crate::m20220101_000001_create_users_table::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(FriendshipStatus::Table)
                    .values([
                        FriendshipStatus::Pending,
                        FriendshipStatus::Accepted,
                        FriendshipStatus::Rejected,
                        FriendshipStatus::Blocked,
                    ])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Friendships::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Friendships::RequesterId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Friendships::RecipientId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-request-user_id")
                            .from(Friendships::Table, Friendships::RequesterId)
                            .to(Users::Table, Users::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-recipient-user_id")
                            .from(Friendships::Table, Friendships::RecipientId)
                            .to(Users::Table, Users::Id),
                    )
                    .col(
                        ColumnDef::new(Friendships::Status)
                            .custom(FriendshipStatus::Table)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Friendships::RequestedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Friendships::RespondedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .primary_key(
                        sea_query::Index::create()
                            .col(Friendships::RequesterId)
                            .col(Friendships::RecipientId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_type(Type::drop().name(FriendshipStatus::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Friendships::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Friendships {
    Table,
    RequesterId,
    RecipientId,
    Status,
    RequestedAt,
    RespondedAt,
}

#[derive(DeriveIden)]
pub enum FriendshipStatus {
    Table,
    Pending,
    Accepted,
    Rejected,
    Blocked,
}
