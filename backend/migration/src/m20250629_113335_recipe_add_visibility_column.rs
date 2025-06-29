use sea_orm_migration::prelude::{extension::postgres::Type, *};

use crate::m20231216_004843_create_recipes_table::Recipes;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(RecipeVisibility::Table)
                    .values([RecipeVisibility::Friends, RecipeVisibility::Private])
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Recipes::Table)
                    .add_column(ColumnDef::new(Recipes::Visibility).custom(RecipeVisibility::Table))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Recipes::Table)
                    .drop_column(Alias::new("visibility"))
                    .to_owned(),
            )
            .await?;

        manager
            .drop_type(Type::drop().name(RecipeVisibility::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum RecipeVisibility {
    Table,
    Friends,
    Private,
}
