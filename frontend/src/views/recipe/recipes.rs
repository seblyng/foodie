use crate::components::not_found::NotFound;
use crate::views::recipe::recipe_image::RecipeImage;
use std::time::Duration;
use thaw::*;

use chrono::{NaiveTime, Timelike};
use common::recipe::Recipe;
use leptos::prelude::*;
use leptos::prelude::{Get, Transition};
use leptos_router::{hooks::use_navigate, NavigateOptions};

use crate::{
    components::loading::Loading,
    context::toast::{use_toast, Toast, ToastType, ToasterTrait},
    request::get,
};

#[component]
pub fn Recipes() -> impl IntoView {
    let toast = use_toast().unwrap();
    let recipes = LocalResource::new(move || async move {
        match get("/api/recipes").send().await {
            Ok(res) => res.json::<Vec<Recipe>>().await.ok(),
            Err(_) => {
                toast.add(Toast {
                    ty: ToastType::Error,
                    body: "Couldn't fetch recipes".to_string(),
                    timeout: Some(Duration::from_secs(5)),
                });
                None
            }
        }
    });

    let _recipes = move || recipes.get().map(|it| it.as_deref().map(|r| r.to_vec()));

    view! {
        <div class="p-4 w-full justify-center flex flex-col items-center">
            <div class="grid grid-cols-12 gap-8">
                <Transition fallback=Loading>
                    {move || {
                        _recipes()
                            .map(|data| match data {
                                None => NotFound.into_any(),
                                Some(r) => {
                                    view! {
                                        <For
                                            each=move || r.clone()
                                            key=|recipe| recipe.id
                                            children=move |recipe| {
                                                view! {
                                                    <div class="col-span-12 sm:col-span-6 lg:col-span-4">
                                                        <RecipeCard recipe=recipe.clone() />
                                                    </div>
                                                }
                                            }
                                        />
                                    }
                                        .into_any()
                                }
                            })
                    }}

                </Transition>

            </div>
        </div>
    }
}

#[component]
fn RecipeCard(recipe: Recipe) -> impl IntoView {
    // TODO: Do I want to include both prep time and baking time when displaying how long time it
    // takes to make the recipe
    let _recipe = recipe.clone();
    let time = move || {
        let total_time = match (_recipe.prep_time, _recipe.baking_time) {
            (Some(prep_time), Some(baking_time)) => NaiveTime::from_hms_opt(
                prep_time.hour() + baking_time.hour(),
                prep_time.minute() + baking_time.minute(),
                0,
            ),
            (Some(prep_time), None) => Some(prep_time),
            (None, Some(baking_time)) => Some(baking_time),
            (None, None) => None,
        };

        total_time.map(format_time).unwrap_or_default()
    };

    // TODO(seb): Should use <a> instead of use_navigate
    let navigate = use_navigate();

    view! {
        <Card
            class="cursor-pointer"
            on:click=move |_| {
                navigate(&format!("/recipes/{}", recipe.id), NavigateOptions::default());
            }
        >
            <CardPreview>
                <RecipeImage src=recipe.img />
            </CardPreview>
            <CardFooter>
                <div class="flex flex-col w-full">
                    <a href=format!("/recipes/{}", recipe.id) class="text-xl font-semibold mb-3">
                        {recipe.name}
                    </a>
                    <CardFooter>
                        <Flex align=FlexAlign::Center>
                            <Icon icon=icondata::AiClockCircleOutlined />
                            <p class="ml-1 flex-none">{time}</p>
                        </Flex>
                        <Flex align=FlexAlign::Center>
                            <Icon icon=icondata::AiShoppingCartOutlined />
                            <p class="ml-1">{format_ingredients(recipe.ingredients.len())}</p>
                        </Flex>
                    </CardFooter>
                </div>
            </CardFooter>
        </Card>
    }
}

// TODO: Probably move these to some common place like a mod.rs file or do something else
// Right now they are duplicated in both here and in the single recipe component
fn format_time(time: NaiveTime) -> String {
    match (time.hour(), time.minute()) {
        (h, m) if h >= 1 && m >= 1 => format!("{h} h {m} min"),
        (h, _) if h >= 1 => format!("{h} h"),
        (_, m) if m >= 1 => format!("{m} min"),
        _ => "".to_string(),
    }
}

fn format_ingredients(len: usize) -> String {
    let val = if len > 1 { "ingredients" } else { "ingredient" };
    format!("{len} {val}")
}
