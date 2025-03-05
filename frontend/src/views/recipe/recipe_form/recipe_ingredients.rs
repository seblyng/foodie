use crate::components::form::form_fields::{
    form_field_input::FormFieldInputType, form_field_select::FormFieldSelect,
};
use crate::components::form::FormGroup;
use crate::components::icons::chevron_down::ChevronDown;
use crate::components::icons::chevron_up::ChevronUp;
use crate::components::icons::close_icon::CloseIcon;
use crate::components::icons::modify_icon::ModifyIcon;
use common::recipe::{CreateRecipe, CreateRecipeIngredient};
use common::strum::IntoEnumIterator;
use leptos::prelude::*;
use rust_decimal::Decimal;

use crate::components::{
    dropdown::DropDownItem, form::form_fields::form_field_input::FormFieldInput,
};

#[component]
pub fn RecipeIngredients() -> impl IntoView {
    let recipe = use_context::<RwSignal<CreateRecipe, LocalStorage>>().unwrap();

    let recipe_ingredient = RwSignal::new(CreateRecipeIngredient::default());

    let units = common::recipe::Unit::iter()
        .map(|u| DropDownItem {
            key: u.to_string(),
            label: u.to_string(),
            value: u,
        })
        .collect::<Vec<_>>();

    // TODO: I want to migrate all this css stuff out to a/some component(s).
    // I want it to just set to 12 cols by default on the outer div.
    // Then I want to add a component that can take the `span` as an optional prop. This should
    // definetely be the case for the `FormField{}`-components, but I need to think of a way to do
    // it with these that are not a separate component.

    view! {
        <div class="card w-full bg-neutral">
            <div class="card-body">
                <h2 class="card-title">"Add ingredients to your recipe"</h2>
                <FormGroup>
                    <FormFieldInput
                        value=move || {
                            recipe_ingredient().amount.map(|a| a.to_string()).unwrap_or_default()
                        }

                        span="col-span-6 md:col-span-3"
                        ty=FormFieldInputType::Number
                        placeholder="Amount"
                        on_input=move |amount| {
                            recipe_ingredient
                                .update(|ri| ri.amount = amount.parse::<Decimal>().ok())
                        }
                    />

                    <FormFieldSelect
                        value=Signal::derive(move || {
                            recipe_ingredient().unit.map(|u| u.to_string()).unwrap_or_default()
                        })

                        span="col-span-6 md:col-span-3"
                        items=units
                        placeholder="Unit"
                        on_change=move |unit| recipe_ingredient.update(|ri| ri.unit = unit)
                    />

                    <FormFieldInput
                        value=move || recipe_ingredient().name
                        span="md:col-span-6"
                        ty=FormFieldInputType::Text
                        placeholder="Name"
                        on_input=move |name| recipe_ingredient.update(|ri| ri.name = name)
                    />
                </FormGroup>
                <button
                    on:click=move |_| {
                        recipe
                            .update(|r| {
                                r.ingredients.push(recipe_ingredient.get_untracked());
                                recipe_ingredient.set(CreateRecipeIngredient::default());
                            })
                    }

                    type="button"
                    class="btn btn-primary"
                >
                    "Add to ingredient list"
                </button>
            </div>
        </div>

        <ul>
            // This is not so good since it rerenders the entire list on each change. However, it was a
            // bit tricky to find a good way to do it with `<For>`, since I want to be able to remove a
            // specific element, and the index is easy to do it. This works for now
            {move || {
                let steps = recipe().ingredients;
                steps
                    .into_iter()
                    .enumerate()
                    .map(|(index, i)| {
                        view! { <Ingredients index=index ingredient=i recipe=recipe/> }
                    })
                    .collect::<Vec<_>>()
            }}

        </ul>
    }
}

#[component]
fn Ingredients(
    index: usize,
    ingredient: CreateRecipeIngredient,
    recipe: RwSignal<CreateRecipe, LocalStorage>,
) -> impl IntoView {
    let num_steps = move || recipe().ingredients.len();
    let remove_card = move |index: usize| {
        recipe.update(|r| {
            r.ingredients.remove(index);
        })
    };

    let swap_card = move |index: usize, other: usize| {
        recipe.update(|r| {
            r.ingredients.swap(index, other);
        })
    };

    view! {
        <li>
            <div class="card w-full bg-neutral">
                <div class="card-body">
                    <div class="card-actions flex justify-between">
                        <div>
                            <h2 class="card-title">
                                {format!(
                                    "{} {} {}",
                                    ingredient.amount.map(|a| a.to_string()).unwrap_or_default(),
                                    ingredient.unit.map(|i| i.to_string()).unwrap_or_default(),
                                    ingredient.name,
                                )}

                            </h2>
                        </div>
                        <div>
                            <Show when=move || { index > 0 }>
                                <button
                                    type="button"
                                    on:click=move |_| swap_card(index, index - 1)
                                    class="btn btn-square btn-sm bg-neutral"
                                >
                                    <ChevronUp/>
                                </button>
                            </Show>
                            <Show when=move || { index < num_steps() - 1 }>
                                <button
                                    type="button"
                                    on:click=move |_| swap_card(index, index + 1)
                                    class="btn btn-square btn-sm bg-neutral"
                                >
                                    <ChevronDown/>
                                </button>
                            </Show>

                            <button
                                type="button"
                                on:click=move |_| remove_card(index)
                                class="btn btn-square btn-sm bg-neutral"
                            >
                                <CloseIcon/>
                            </button>
                            <button
                                type="button"
                                on:click=move |_| {}
                                class="btn btn-square btn-sm bg-neutral"
                            >
                                <ModifyIcon/>
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </li>
    }
}
