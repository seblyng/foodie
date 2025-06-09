use sea_orm_migration::prelude::*;

use crate::{
    m20220101_000001_create_users_table::Users, m20231216_004843_create_recipes_table::Recipes,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RecipeShare::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RecipeShare::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(RecipeShare::RecipeId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-recipe-share-recipe_id")
                            .from(RecipeShare::Table, RecipeShare::RecipeId)
                            .to(Recipes::Table, Recipes::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(RecipeShare::SharedWithId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-recipe-share-user_id")
                            .from(RecipeShare::Table, RecipeShare::SharedWithId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(RecipeShare::CreatedAt)
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
            .drop_table(Table::drop().table(RecipeShare::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum RecipeShare {
    Table,
    Id,
    RecipeId,
    SharedWithId,
    CreatedAt,
}
