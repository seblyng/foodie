use sea_orm_migration::prelude::*;

use crate::m20220101_000001_create_users_table::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Recipes::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Recipes::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Recipes::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-recipe-user_id")
                            .from(Recipes::Table, Recipes::UserId)
                            .to(Users::Table, Users::Id),
                    )
                    .col(ColumnDef::new(Recipes::Name).string().not_null())
                    .col(ColumnDef::new(Recipes::Description).string())
                    .col(ColumnDef::new(Recipes::Instructions).string())
                    .col(ColumnDef::new(Recipes::Img).string())
                    .col(ColumnDef::new(Recipes::Servings).integer().not_null())
                    .col(ColumnDef::new(Recipes::PrepTime).time())
                    .col(ColumnDef::new(Recipes::BakingTime).time())
                    .col(
                        ColumnDef::new(Recipes::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Recipes::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Recipes::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Recipes {
    Table,
    Id,
    UserId,
    Name,
    Description,
    Instructions,
    Img,
    Servings,
    PrepTime,
    BakingTime,
    CreatedAt,
    UpdatedAt,
    Visibility,
}
