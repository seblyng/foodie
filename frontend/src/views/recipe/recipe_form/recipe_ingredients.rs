use crate::components::form::form_fields::form_field_combobox::FormFieldSelect;
use crate::components::form::form_fields::form_field_input::FormFieldInput;
use crate::components::form::form_fields::form_field_number_input::FormFieldNumberInput;
use crate::components::form::FormGroup;
use common::recipe::{CreateRecipe, CreateRecipeIngredient, Unit};
use common::strum::IntoEnumIterator;
use leptos::prelude::*;
use rust_decimal::Decimal;
use thaw::*;

#[component]
pub fn RecipeIngredients() -> impl IntoView {
    let recipe = use_context::<RwSignal<CreateRecipe>>().unwrap();

    let recipe_ingredient = RwSignal::new(CreateRecipeIngredient::default());

    let name = slice!(recipe_ingredient.name);
    let amount = RwSignal::new(String::new());

    Effect::new(move || {
        let Ok(a) = amount().parse::<Decimal>() else {
            return;
        };
        recipe_ingredient.update(|ri| {
            ri.amount = Some(a);
        });
    });

    let selected = create_slice(
        recipe_ingredient,
        |ri| ri.unit.map(|u| u.to_string()).unwrap_or_default(),
        |ri, n: String| ri.unit = n.parse::<Unit>().ok(),
    );

    view! {
        <FormGroup>
            <FormFieldNumberInput
                name="amount"
                class="md:col-span-3 col-span-6"
                placeholder="Amount"
                value=amount
            />

            <FormFieldSelect class="md:col-span-3 col-span-6" value=selected placeholder="Unit">
                {move || {
                    common::recipe::Unit::iter()
                        .map(|u| {
                            view! {
                                <ComboboxOption text=u.to_string() value=u.to_string()>
                                    {u.to_string()}
                                </ComboboxOption>
                            }
                        })
                        .collect::<Vec<_>>()
                }}
            </FormFieldSelect>

            <FormFieldInput class="col-span-12 md:col-span-6" value=name placeholder="Name" />
            <Button
                class="col-span-12"
                button_type=ButtonType::Button
                on:click=move |_| {
                    recipe
                        .update(|r| {
                            r.ingredients.push(recipe_ingredient.get_untracked());
                            recipe_ingredient.set(CreateRecipeIngredient::default());
                            amount.set(String::new());
                        })
                }
            >

                "Add to ingredient list"
            </Button>
            <ul class="col-span-12">
                // This is not so good since it rerenders the entire list on each change. However, it was a
                // bit tricky to find a good way to do it with `<For>`, since I want to be able to remove a
                // specific element, and the index is easy to do it. This works for now
                {move || {
                    let steps = recipe().ingredients;
                    steps
                        .into_iter()
                        .enumerate()
                        .map(|(index, i)| {
                            view! {
                                <li>
                                    <Ingredients index=index ingredient=i recipe=recipe />
                                </li>
                            }
                        })
                        .collect::<Vec<_>>()
                }}

            </ul>
        </FormGroup>
    }
}

#[component]
fn Ingredients(
    index: usize,
    ingredient: CreateRecipeIngredient,
    recipe: RwSignal<CreateRecipe>,
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
        <Card>
            <CardHeader>
                <h2 class="card-title">
                    {format!(
                        "{} {} {}",
                        ingredient.amount.map(|a| a.to_string()).unwrap_or_default(),
                        ingredient.unit.map(|i| i.to_string()).unwrap_or_default(),
                        ingredient.name,
                    )}

                </h2>
                <CardHeaderAction slot>
                    <Show when=move || { index > 0 }>
                        <Button
                            button_type=ButtonType::Button
                            appearance=ButtonAppearance::Transparent
                            icon=icondata::BiChevronUpRegular
                            on:click=move |_| swap_card(index, index - 1)
                        />
                    </Show>
                    <Show when=move || { index < num_steps() - 1 }>
                        <Button
                            button_type=ButtonType::Button
                            appearance=ButtonAppearance::Transparent
                            icon=icondata::BiChevronDownRegular
                            on:click=move |_| swap_card(index, index + 1)
                        />
                    </Show>
                    <Button
                        button_type=ButtonType::Button
                        appearance=ButtonAppearance::Transparent
                        icon=icondata::AiCloseOutlined
                        on:click=move |_| remove_card(index)
                    />
                </CardHeaderAction>
            </CardHeader>
        </Card>
    }
}
