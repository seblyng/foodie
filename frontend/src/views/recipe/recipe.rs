use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::{use_navigate, use_params_map};
use num::rational::Ratio;
use std::ops::{Add, Sub};
use std::time::Duration;
use thaw::*;

use crate::components::loading::Loading;
use crate::components::not_found::NotFound;
use crate::context::auth::AuthStore;
use crate::context::toast::{use_toast, Toast, ToastType, ToasterTrait};
use crate::views::recipe::recipe_image::RecipeImage;
use crate::views::recipe::{format_ingredients, format_time, total_time};
use common::recipe::{Recipe, RecipeIngredient};
use leptos_router::NavigateOptions;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;

use crate::request::{delete, get};

#[component]
pub fn RecipePage() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|params| params.get("id").unwrap_or_default());

    let recipe = LocalResource::new(move || async move {
        get(&format!("/api/recipes/{}", id()))
            .send()
            .await
            .ok()?
            .json::<Recipe>()
            .await
            .ok()
    });

    let _recipe = move || recipe.get().as_deref().map(|it| it.to_owned());

    view! {
        <Transition fallback=Loading>
            {move || {
                _recipe()
                    .map(|data| match data {
                        None => NotFound.into_any(),
                        Some(r) => view! { <Recipe recipe=r /> }.into_any(),
                    })
            }}

        </Transition>
    }
}

#[component]
fn Recipe(recipe: Recipe) -> impl IntoView {
    view! {
        <div class="mx-auto w-[80%] max-w-screen-xl">
            <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
                <div class="lg:col-span-2 order-1 lg:order-2">
                    <div class="flex flex-col gap-8">
                        <RecipeCard recipe=recipe.clone() />
                        {if let Some(ref steps) = recipe.instructions {
                            view! { <RecipeSteps steps=steps.clone() /> }.into_any()
                        } else {
                            ().into_any()
                        }}
                    </div>
                </div>

                <div class="lg:col-span-1 order-2 lg:order-1">
                    <RecipeIngredients
                        recipe=recipe.clone()
                        ingredients=recipe.ingredients.clone()
                    />
                </div>
            </div>
        </div>
    }
}

#[component]
fn RecipeCard(recipe: Recipe) -> impl IntoView {
    let state = expect_context::<AuthStore>();

    let _recipe = recipe.clone();
    let toast = use_toast().unwrap();
    let time = move || {
        total_time(_recipe.prep_time, _recipe.baking_time)
            .map(format_time)
            .unwrap_or_default()
    };
    let open = RwSignal::new(false);

    let navigate = use_navigate();

    let on_select = move |key: String| {
        let nav = navigate.clone();
        match key.as_ref() {
            "edit" => nav(&format!("/recipes/{}/edit", recipe.id), Default::default()),
            "delete" => open.set(true),
            _ => unreachable!(),
        };
    };

    let navigate = use_navigate();
    let on_delete = move |_| {
        let nav = navigate.clone();
        spawn_local(async move {
            match delete(&format!("/api/recipes/{}", recipe.id)).send().await {
                Ok(r) if r.ok() => {
                    toast.add(Toast {
                        ty: ToastType::Success,
                        body: "Successfully deleted recipe!".to_string(),
                        timeout: Some(Duration::from_secs(5)),
                    });
                    nav("/recipes", NavigateOptions::default());
                }
                _ => {
                    toast.add(Toast {
                        ty: ToastType::Error,
                        body: "Failed to delete recipe".to_string(),
                        timeout: Some(Duration::from_secs(5)),
                    });
                }
            };
        });
    };

    view! {
        <Card>
            <CardHeader>
                <Body1>{recipe.name}</Body1>
                <CardHeaderAction slot>
                    {if state.id == recipe.user_id {
                        view! {
                            <Menu position=MenuPosition::BottomEnd on_select=on_select>
                                <MenuTrigger slot>
                                    <Button
                                        appearance=ButtonAppearance::Transparent
                                        icon=icondata::AiMoreOutlined
                                    />
                                </MenuTrigger>
                                <MenuItem value="edit">"Edit"</MenuItem>
                                <MenuItem value="delete">"Delete"</MenuItem>
                            </Menu>
                        }
                            .into_any()
                    } else {
                        ().into_any()
                    }} <Dialog open>
                        <DialogSurface>
                            <DialogBody>
                                <DialogTitle>"Delete recipe"</DialogTitle>
                                <DialogContent>
                                    "Are you sure you want to delete the recipe?"
                                </DialogContent>
                                <DialogActions>
                                    <Button on:click=on_delete appearance=ButtonAppearance::Primary>
                                        "Yes"
                                    </Button>
                                    <Button on:click=move |_| open.set(false)>"No"</Button>
                                </DialogActions>
                            </DialogBody>
                        </DialogSurface>
                    </Dialog>
                </CardHeaderAction>
            </CardHeader>
            <CardPreview>
                <RecipeImage src=recipe.img />
            </CardPreview>
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
        </Card>
    }
}

#[component]
fn RecipeIngredients(recipe: Recipe, ingredients: Vec<RecipeIngredient>) -> impl IntoView {
    let internal_ingredients = RwSignal::new(ingredients.clone());
    let (servings, set_servings) = signal(Ratio::new(recipe.servings as i128, 1));

    let set_ingredients = move |old_serving: Ratio<i128>, new_serving: Ratio<i128>| {
        if new_serving.to_integer() < 0 {
            return;
        }

        let new_serving = if new_serving.to_integer() == 0 {
            Ratio::new(1, 2)
        } else if old_serving == Ratio::new(1, 2) {
            Ratio::new(1, 1)
        } else {
            new_serving
        };

        let new_ingredients = internal_ingredients()
            .iter()
            .map(|i| RecipeIngredient {
                ingredient_id: i.ingredient_id,
                ingredient_name: i.ingredient_name.clone(),
                unit: i.unit,
                amount: i
                    .amount
                    .map(|a| compute_amount(a, old_serving, new_serving)),
            })
            .collect();

        internal_ingredients.set(new_ingredients);
        set_servings(new_serving);
    };

    view! {
        <Card>
            <h1 class="flex text-2xl justify-center">"Ingredients"</h1>
            <Flex justify=FlexJustify::Center>
                <Button
                    appearance=ButtonAppearance::Transparent
                    icon=icondata::AiMinusCircleOutlined
                    on:click=move |_| { set_ingredients(servings(), servings().sub(1)) }
                />
                <p>{move || format!("{} servings", servings())}</p>
                <Button
                    appearance=ButtonAppearance::Transparent
                    icon=icondata::AiPlusCircleOutlined
                    on:click=move |_| { set_ingredients(servings(), servings().add(1)) }
                />
            </Flex>
            {move || {
                internal_ingredients()
                    .into_iter()
                    .map(|ingredient| {
                        view! {
                            <p class="mb-1 bg-neutral rounded-md">
                                {format!(
                                    "{} {} {}",
                                    ingredient
                                        .amount
                                        .map(|a| {
                                            if a.is_integer() {
                                                a.to_i64().unwrap().to_string()
                                            } else {
                                                a.to_string()
                                            }
                                        })
                                        .unwrap_or_default(),
                                    ingredient.unit.map(|i| i.to_string()).unwrap_or_default(),
                                    ingredient.ingredient_name,
                                )}

                            </p>
                        }
                    })
                    .collect::<Vec<_>>()
            }}

        </Card>
    }
}

#[component]
fn RecipeSteps(steps: Vec<String>) -> impl IntoView {
    view! {
        <Card>
            <h1 class="text-2xl pb-4">"Steps"</h1>
            {steps
                .into_iter()
                .enumerate()
                .map(|(idx, step)| {
                    view! {
                        <Flex align=FlexAlign::Center>
                            <Badge
                                class="shrink-0"
                                size=BadgeSize::ExtraLarge
                                appearance=BadgeAppearance::Filled
                                color=BadgeColor::Brand
                            >
                                {idx + 1}
                            </Badge>
                            <p>{step}</p>
                        </Flex>
                    }
                })
                .collect::<Vec<_>>()}
        </Card>
    }
}

fn compute_amount(amount: Decimal, old_serving: Ratio<i128>, new_serving: Ratio<i128>) -> Decimal {
    let ratio_amount = Ratio::new(amount.mantissa(), 10i32.pow(amount.scale()) as i128)
        / old_serving
        * new_serving;
    Decimal::from_i128_with_scale(*ratio_amount.numer(), 0)
        / Decimal::from_i128_with_scale(*ratio_amount.denom(), 0)
}
