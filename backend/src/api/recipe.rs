use crate::{
    app::AppState,
    auth_backend::AuthSession,
    entities::{ingredients, recipe_ingredients, recipe_share, recipes, sea_orm_active_enums},
    storage::FoodieStorage,
    ApiError,
};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use common::recipe::{CreateRecipe, Recipe, RecipeImage, RecipeIngredient};
use futures_util::{future::join_all, StreamExt};
use hyper::Method;
use sea_orm::{
    sea_query::OnConflict, ActiveValue::NotSet, ColumnTrait, ConnectionTrait, DatabaseConnection,
    EntityTrait, LoaderTrait, QueryFilter, Set, StreamTrait, TransactionTrait,
};
use uuid::Uuid;

// Creates a recipe. Dependant on that the ingredients are already created
pub async fn post_recipe<T>(
    auth: AuthSession,
    State(db): State<DatabaseConnection>,
    State(state): State<AppState<T>>,
    Json(recipe): Json<CreateRecipe>,
) -> Result<Json<Recipe>, ApiError>
where
    T: FoodieStorage + Send + Sync + Clone,
{
    let user = auth.user.unwrap();

    let created_ingredients = create_ingredients(&recipe, user.id, &db).await?;

    let tx = db.begin().await?;

    let created_recipe = recipes::Entity::insert(recipes::ActiveModel {
        id: NotSet,
        user_id: Set(user.id),
        name: Set(recipe.name),
        description: Set(recipe.description),
        instructions: Set(recipe.instructions),
        img: Set(recipe.img),
        servings: Set(recipe.servings),
        prep_time: Set(recipe.prep_time),
        baking_time: Set(recipe.baking_time),
        created_at: NotSet,
        updated_at: NotSet,
    })
    .exec_with_returning(&tx)
    .await?;

    let shared_recipes = recipe
        .shared_with
        .iter()
        .map(|id| recipe_share::ActiveModel {
            id: NotSet,
            recipe_id: Set(created_recipe.id),
            created_at: NotSet,
            shared_with_id: Set(*id),
        })
        .collect::<Vec<_>>();

    if !shared_recipes.is_empty() {
        recipe_share::Entity::insert_many(shared_recipes)
            .exec(&tx)
            .await?;
    }

    let models = created_ingredients.into_iter().filter_map(|i| {
        let ri = recipe.ingredients.iter().find(|ri| ri.name == i.name)?;
        Some(recipe_ingredients::ActiveModel {
            recipe_id: Set(created_recipe.id),
            ingredient_id: Set(i.id),
            unit: Set(ri.unit.map(|u| u.into())),
            amount: Set(ri.amount),
        })
    });

    recipe_ingredients::Entity::insert_many(models)
        .exec(&tx)
        .await?;

    tx.commit().await?;

    let ingredients = get_recipe_ingredients(&db, created_recipe.id).await?;

    let recipe_image = get_presigned_url_for_get(state.storage, created_recipe.img).await?;

    Ok(Json(Recipe {
        id: created_recipe.id,
        user_id: created_recipe.user_id,
        name: created_recipe.name,
        description: created_recipe.description,
        instructions: created_recipe.instructions,
        img: recipe_image,
        servings: created_recipe.servings,
        updated_at: created_recipe.updated_at,
        prep_time: created_recipe.prep_time,
        baking_time: created_recipe.baking_time,
        shared_with: recipe.shared_with,
        ingredients,
    }))
}

// Gets a recipe with an id
pub async fn get_recipe<T>(
    auth: AuthSession,
    State(state): State<AppState<T>>,
    Path(recipe_id): Path<i32>,
) -> Result<Json<Recipe>, ApiError>
where
    T: FoodieStorage + Send + Sync + Clone,
{
    let user = auth.user.unwrap();

    let recipe_model = recipes::Entity::find_by_id(recipe_id)
        .filter(recipes::Column::UserId.eq(user.id))
        .one(&state.db)
        .await?
        .ok_or(ApiError::RecordNotFound)?;

    let shared_with = recipe_share::Entity::find()
        .filter(recipe_share::Column::RecipeId.eq(recipe_id))
        .all(&state.db)
        .await?
        .iter()
        .map(|it| it.shared_with_id)
        .collect::<Vec<_>>();

    let ingredients = get_recipe_ingredients(&state.db, recipe_model.id).await?;

    let recipe_image = get_presigned_url_for_get(state.storage, recipe_model.img).await?;

    Ok(Json(Recipe {
        id: recipe_model.id,
        user_id: recipe_model.user_id,
        name: recipe_model.name,
        description: recipe_model.description,
        instructions: recipe_model.instructions,
        img: recipe_image,
        servings: recipe_model.servings,
        updated_at: recipe_model.updated_at,
        prep_time: recipe_model.prep_time,
        baking_time: recipe_model.baking_time,
        shared_with,
        ingredients,
    }))
}

async fn _get_recipes<T>(
    recipes: Vec<(recipes::Model, Vec<recipe_share::Model>)>,
    state: AppState<T>,
) -> Result<Json<Vec<Recipe>>, ApiError>
where
    T: FoodieStorage + Send + Sync + Clone,
{
    let rec = recipes
        .iter()
        .map(|r| r.0.clone().to_owned())
        .collect::<Vec<_>>();

    let ingredients = rec
        .load_many_to_many(ingredients::Entity, recipe_ingredients::Entity, &state.db)
        .await?;

    let ingredients_with_units = rec.load_many(recipe_ingredients::Entity, &state.db).await?;

    let recipes = recipes
        .into_iter()
        .zip(
            ingredients
                .into_iter()
                .zip(ingredients_with_units.into_iter()),
        )
        .map(|r| {
            let state = state.storage.clone();
            async move {
                let ingredients =
                    r.1 .0
                        .into_iter()
                        .zip(r.1 .1.into_iter())
                        .map(|i| RecipeIngredient {
                            ingredient_id: i.0.id,
                            ingredient_name: i.0.name,
                            unit: i.1.unit.map(|u| u.into()),
                            amount: i.1.amount,
                        })
                        .collect();

                let recipe_image = get_presigned_url_for_get(state, r.0 .0.img)
                    .await
                    .ok()
                    .unwrap_or_default();

                Recipe {
                    id: r.0 .0.id,
                    user_id: r.0 .0.user_id,
                    name: r.0 .0.name,
                    description: r.0 .0.description,
                    instructions: r.0 .0.instructions,
                    img: recipe_image,
                    servings: r.0 .0.servings,
                    updated_at: r.0 .0.updated_at,
                    prep_time: r.0 .0.prep_time,
                    baking_time: r.0 .0.baking_time,
                    shared_with: r.0 .1.iter().map(|it| it.shared_with_id).collect(),
                    ingredients,
                }
            }
        })
        .collect::<Vec<_>>();

    let recipes = join_all(recipes).await;

    Ok(Json(recipes))
}

// Gets all the recipes for the user
pub async fn get_recipes<T>(
    auth: AuthSession,
    State(state): State<AppState<T>>,
) -> Result<Json<Vec<Recipe>>, ApiError>
where
    T: FoodieStorage + Send + Sync + Clone,
{
    let user = auth.user.unwrap();
    let recipes = recipes::Entity::find()
        .filter(recipes::Column::UserId.eq(user.id))
        .find_with_related(recipe_share::Entity)
        .all(&state.db)
        .await?;

    _get_recipes(recipes, state).await
}

pub async fn update_recipe<T>(
    auth: AuthSession,
    Path(recipe_id): Path<i32>,
    State(db): State<DatabaseConnection>,
    State(state): State<AppState<T>>,
    Json(recipe): Json<CreateRecipe>,
) -> Result<Json<Recipe>, ApiError>
where
    T: FoodieStorage + Send + Sync + Clone,
{
    let user = auth.user.unwrap();

    let created_ingredients = create_ingredients(&recipe, user.id, &db).await?;

    let tx = db.begin().await?;

    let updated_recipe = recipes::Entity::update(recipes::ActiveModel {
        id: Set(recipe_id),
        user_id: NotSet,
        name: Set(recipe.name),
        description: Set(recipe.description),
        instructions: Set(recipe.instructions),
        img: Set(recipe.img),
        servings: Set(recipe.servings),
        prep_time: Set(recipe.prep_time),
        baking_time: Set(recipe.baking_time),
        created_at: NotSet,
        updated_at: Set(chrono::Utc::now().into()),
    })
    .filter(recipes::Column::Id.eq(recipe_id))
    .filter(recipes::Column::UserId.eq(user.id))
    .exec(&tx)
    .await?;

    let models = created_ingredients.into_iter().filter_map(|i| {
        let ri = recipe.ingredients.iter().find(|ri| ri.name == i.name)?;
        Some(recipe_ingredients::ActiveModel {
            recipe_id: Set(updated_recipe.id),
            ingredient_id: Set(i.id),
            unit: Set(ri.unit.map(|u| u.into())),
            amount: Set(ri.amount),
        })
    });

    recipe_ingredients::Entity::delete_many()
        .filter(recipe_ingredients::Column::RecipeId.eq(recipe_id))
        .exec(&tx)
        .await?;

    recipe_ingredients::Entity::insert_many(models)
        .exec(&tx)
        .await?;

    tx.commit().await?;

    let ingredients = get_recipe_ingredients(&db, recipe_id).await?;

    let recipe_image = get_presigned_url_for_get(state.storage, updated_recipe.img).await?;

    Ok(Json(Recipe {
        id: updated_recipe.id,
        user_id: updated_recipe.user_id,
        name: updated_recipe.name,
        description: updated_recipe.description,
        instructions: updated_recipe.instructions,
        img: recipe_image,
        servings: updated_recipe.servings,
        updated_at: updated_recipe.updated_at,
        prep_time: updated_recipe.prep_time,
        baking_time: updated_recipe.baking_time,
        shared_with: recipe.shared_with,
        ingredients,
    }))
}

pub async fn delete_recipe<T>(
    auth: AuthSession,
    State(db): State<DatabaseConnection>,
    State(state): State<AppState<T>>,
    Path(recipe_id): Path<i32>,
) -> Result<impl IntoResponse, ApiError>
where
    T: FoodieStorage + Send + Sync + Clone,
{
    let user = auth.user.unwrap();

    let recipe = recipes::Entity::find_by_id(recipe_id)
        .filter(recipes::Column::UserId.eq(user.id))
        .one(&db)
        .await?;

    if let Some(recipe) = recipe {
        if let Some(img) = recipe.img {
            let _ = state.storage.delete(img).await;
        }
    }

    recipes::Entity::delete_by_id(recipe_id)
        .filter(recipes::Column::UserId.eq(user.id))
        .exec(&db)
        .await?;

    Ok(())
}

async fn create_ingredients(
    recipe: &CreateRecipe,
    user_id: i32,
    db: &DatabaseConnection,
) -> Result<Vec<ingredients::Model>, anyhow::Error> {
    let ingredients: (Vec<String>, Vec<ingredients::ActiveModel>) = recipe
        .ingredients
        .iter()
        .map(|i| {
            (
                i.name.clone(),
                ingredients::ActiveModel {
                    id: NotSet,
                    name: Set(i.name.clone()),
                    user_id: Set(user_id),
                },
            )
        })
        .unzip();

    ingredients::Entity::insert_many(ingredients.1)
        .on_conflict(
            OnConflict::column(ingredients::Column::Name)
                .update_column(ingredients::Column::Name)
                .to_owned(),
        )
        .exec(db)
        .await?;

    Ok(ingredients::Entity::find()
        .filter(ingredients::Column::Name.is_in(ingredients.0))
        .all(db)
        .await?)
}

pub async fn get_presigned_url_for_upload<T>(
    State(state): State<AppState<T>>,
) -> Result<Json<RecipeImage>, ApiError>
where
    T: FoodieStorage + Send + Sync + Clone,
{
    let name = Uuid::new_v4();
    let url = state.storage.get_presigned_url(name, Method::PUT).await?;

    Ok(Json(RecipeImage { id: name, url }))
}

async fn get_presigned_url_for_get<T>(
    storage: T,
    image: Option<Uuid>,
) -> Result<Option<String>, anyhow::Error>
where
    T: FoodieStorage,
{
    match image {
        Some(img) => Ok(Some(storage.get_presigned_url(img, Method::GET).await?)),
        None => Ok(None),
    }
}

async fn get_recipe_ingredients<C>(
    db: &C,
    recipe_id: i32,
) -> Result<Vec<RecipeIngredient>, anyhow::Error>
where
    C: ConnectionTrait + Send + StreamTrait,
{
    let ingredients = recipe_ingredients::Entity::find()
        .filter(recipe_ingredients::Column::RecipeId.eq(recipe_id))
        .find_also_related(ingredients::Entity)
        .stream(db)
        .await?
        .map(|i| {
            let i = i.unwrap();
            RecipeIngredient {
                ingredient_id: i.0.ingredient_id,
                ingredient_name: i.1.unwrap().name,
                unit: i.0.unit.map(|u| u.into()),
                amount: i.0.amount,
            }
        })
        .collect::<Vec<_>>()
        .await;

    Ok(ingredients)
}

// Gets all the recipes shared with the users
pub async fn get_shared_recipes<T>(
    auth: AuthSession,
    State(state): State<AppState<T>>,
) -> Result<Json<Vec<Recipe>>, ApiError>
where
    T: FoodieStorage + Send + Sync + Clone,
{
    let user = auth.user.unwrap();
    let shared_recipes = recipes::Entity::find()
        .find_with_related(recipe_share::Entity)
        .filter(recipe_share::Column::SharedWithId.eq(user.id))
        .all(&state.db)
        .await?;

    _get_recipes(shared_recipes, state).await
}

macro_rules! convert_unit {
    ($first:ty, $second: ty) => {
        impl From<$first> for $second {
            fn from(value: $first) -> Self {
                match value {
                    <$first>::Milligram => <$second>::Milligram,
                    <$first>::Gram => <$second>::Gram,
                    <$first>::Hectogram => <$second>::Hectogram,
                    <$first>::Kilogram => <$second>::Kilogram,
                    <$first>::Milliliter => <$second>::Milliliter,
                    <$first>::Deciliter => <$second>::Deciliter,
                    <$first>::Liter => <$second>::Liter,
                    <$first>::Teaspoon => <$second>::Teaspoon,
                    <$first>::Tablespoon => <$second>::Tablespoon,
                    <$first>::Cup => <$second>::Cup,
                    <$first>::Clove => <$second>::Clove,
                    <$first>::Pinch => <$second>::Pinch,
                }
            }
        }
    };
}

convert_unit!(common::recipe::Unit, sea_orm_active_enums::Unit);
convert_unit!(sea_orm_active_enums::Unit, common::recipe::Unit);
