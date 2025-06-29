pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_users_table;
mod m20231216_004843_create_recipes_table;
mod m20231216_103342_create_unit_type;
mod m20231216_103916_create_ingredients_table;
mod m20231216_104607_create_recipe_ingredients_table;
mod m20240218_134359_instructions_as_vec;
mod m20240331_125419_image_to_uuid_column;
mod m20250609_144836_add_friends_table;
mod m20250609_174627_add_unique_user_pair_index;
mod m20250609_200736_add_recipe_share_table;
mod m20250629_113335_recipe_add_visibility_column;
mod m20250629_115347_recipe_visibilit_default_value;
mod m20250629_120028_set_recipe_visibility_non_null;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_users_table::Migration),
            Box::new(m20231216_004843_create_recipes_table::Migration),
            Box::new(m20231216_103342_create_unit_type::Migration),
            Box::new(m20231216_103916_create_ingredients_table::Migration),
            Box::new(m20231216_104607_create_recipe_ingredients_table::Migration),
            Box::new(m20240218_134359_instructions_as_vec::Migration),
            Box::new(m20240331_125419_image_to_uuid_column::Migration),
            Box::new(m20250609_144836_add_friends_table::Migration),
            Box::new(m20250609_174627_add_unique_user_pair_index::Migration),
            Box::new(m20250609_200736_add_recipe_share_table::Migration),
            Box::new(m20250629_113335_recipe_add_visibility_column::Migration),
            Box::new(m20250629_115347_recipe_visibilit_default_value::Migration),
            Box::new(m20250629_120028_set_recipe_visibility_non_null::Migration),
        ]
    }
}
