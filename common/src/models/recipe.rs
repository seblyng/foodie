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
    pub visibility: RecipeVisibility,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default, Display, EnumIter)]
pub enum RecipeVisibility {
    #[default]
    Friends,
    Private,
}

impl FromStr for RecipeVisibility {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "Friends" => Ok(Self::Friends),
            "Private" => Ok(Self::Private),
            _ => Err(()),
        }
    }
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
    pub visibility: RecipeVisibility,
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
impl FromStr for Unit {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "Milligram" => Ok(Self::Milligram),
            "Gram" => Ok(Self::Gram),
            "Hectogram" => Ok(Self::Hectogram),
            "Kilogram" => Ok(Self::Kilogram),
            "Milliliter" => Ok(Self::Milliliter),
            "Deciliter" => Ok(Self::Deciliter),
            "Liter" => Ok(Self::Liter),
            "Teaspoon" => Ok(Self::Teaspoon),
            "Tablespoon" => Ok(Self::Tablespoon),
            "Cup" => Ok(Self::Cup),
            "Clove" => Ok(Self::Clove),
            "Pinch" => Ok(Self::Pinch),
            _ => Err(()),
        }
    }
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
            visibility: recipe.visibility,
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
