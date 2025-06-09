use std::str::FromStr;

use chrono::{DateTime, FixedOffset, NaiveTime};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CreateRecipe {
    pub name: String,
    pub description: Option<String>,
    pub instructions: Option<Vec<String>>,
    pub img: Option<Uuid>,
    pub servings: i32,
    pub prep_time: Option<NaiveTime>,
    pub baking_time: Option<NaiveTime>,
    pub ingredients: Vec<CreateRecipeIngredient>,
    // user_ids to share the recipe with
    pub shared_with: Vec<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Recipe {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub instructions: Option<Vec<String>>,
    pub img: Option<String>,
    pub servings: i32,
    pub updated_at: DateTime<FixedOffset>,
    pub prep_time: Option<NaiveTime>,
    pub baking_time: Option<NaiveTime>,
    pub ingredients: Vec<RecipeIngredient>,
    pub shared_with: Vec<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy, Eq, PartialEq, EnumIter, Display)]
pub enum Unit {
    Milligram,
    Gram,
    Hectogram,
    Kilogram,
    Milliliter,
    Deciliter,
    Liter,
    Teaspoon,
    Tablespoon,
    Cup,
    Clove,
    Pinch,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct CreateRecipeIngredient {
    pub name: String,
    pub unit: Option<Unit>,
    pub amount: Option<Decimal>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct RecipeIngredient {
    pub ingredient_id: i32,
    pub ingredient_name: String,
    pub unit: Option<Unit>,
    pub amount: Option<Decimal>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct RecipeImage {
    pub id: Uuid,
    pub url: String,
}

impl From<Recipe> for CreateRecipe {
    fn from(recipe: Recipe) -> Self {
        // Hack to get the image id (name) from the presigned url
        let img = recipe.img.map(|i| {
            let rest = i.split_once("aws/").unwrap();
            let id = rest.1.chars().take(36).collect::<String>();
            Uuid::from_str(&id).unwrap()
        });

        Self {
            name: recipe.name,
            description: recipe.description,
            instructions: recipe.instructions,
            img,
            servings: recipe.servings,
            prep_time: recipe.prep_time,
            baking_time: recipe.baking_time,
            shared_with: recipe.shared_with,
            ingredients: recipe
                .ingredients
                .into_iter()
                .map(CreateRecipeIngredient::from)
                .collect(),
        }
    }
}

impl From<RecipeIngredient> for CreateRecipeIngredient {
    fn from(recipe_ingredient: RecipeIngredient) -> Self {
        Self {
            name: recipe_ingredient.ingredient_name,
            unit: recipe_ingredient.unit,
            amount: recipe_ingredient.amount,
        }
    }
}
