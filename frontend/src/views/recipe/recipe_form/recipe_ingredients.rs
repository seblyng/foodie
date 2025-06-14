use crate::components::form::FormGroup;
use common::recipe::{CreateRecipe, CreateRecipeIngredient, Unit};
use common::strum::IntoEnumIterator;
use leptos::prelude::*;
use num::ToPrimitive;
use rust_decimal::Decimal;
use thaw::*;

#[component]
pub fn RecipeIngredients() -> impl IntoView {
    let recipe = use_context::<RwSignal<CreateRecipe>>().unwrap();

    let recipe_ingredient = RwSignal::new(CreateRecipeIngredient::default());

    let _units = move || {
        common::recipe::Unit::iter()
            .map(|u| {
                view! { <ComboboxOption value=u.to_string() text=u.to_string() /> }
            })
            .collect::<Vec<_>>()
    };

    // TODO: I want to migrate all this css stuff out to a/some component(s).
    // I want it to just set to 12 cols by default on the outer div.
    // Then I want to add a component that can take the `span` as an optional prop. This should
    // definetely be the case for the `FormField{}`-components, but I need to think of a way to do
    // it with these that are not a separate component.
    // let amount = slice!(recipe.amount);
    let name = slice!(recipe_ingredient.name);
    let amount = create_slice(
        recipe_ingredient,
        |ri| Decimal::to_f64(&ri.amount.unwrap_or_default()).unwrap(),
        |ri, n: f64| ri.amount = Decimal::from_f64_retain(n),
    );
    let selected = create_slice(
        recipe_ingredient,
        |ri| ri.unit.map(|u| u.to_string()).unwrap_or_default(),
        |ri, n: String| ri.unit = n.parse::<Unit>().ok(),
    );

    view! {
        <div class="card w-full bg-neutral">
            <div class="card-body">
                <h2 class="card-title">"Add ingredients to your recipe"</h2>
                <FormGroup>
                    <div class="col-span-6 md:col-span-3">
                        <SpinButton<
                        f64,
                    >
                            placeholder="Servings"
                            class="w-full"
                            value=amount
                            step_page=1.0
                            min=0.0
                            max=72.0
                        />
                    </div>

                    <Combobox class="col-span-6 md:col-span-3" value=selected placeholder="Unit">
                        {_units}
                    </Combobox>

                    <Input class="md:col-span-6" value=name placeholder="Name" />
                </FormGroup>
                <Button
                    class="col-span-12"
                    button_type=ButtonType::Button
                    on:click=move |_| {
                        recipe
                            .update(|r| {
                                r.ingredients.push(recipe_ingredient.get_untracked());
                                recipe_ingredient.set(CreateRecipeIngredient::default());
                            })
                    }
                >

                    "Add to ingredient list"
                </Button>
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
                        view! { <Ingredients index=index ingredient=i recipe=recipe /> }
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
        <li>
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
                        <Button
                            button_type=ButtonType::Button
                            appearance=ButtonAppearance::Transparent
                            icon=icondata::AiEditOutlined
                            on:click=move |_| {}
                        />
                    </CardHeaderAction>
                </CardHeader>
            </Card>
        </li>
    }
}
