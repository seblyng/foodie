use crate::components::form::{form_fields::form_field_textarea::FormFieldTextarea, FormGroup};
use common::recipe::CreateRecipe;
use leptos::prelude::*;
use thaw::*;

#[component]
pub fn RecipeSteps() -> impl IntoView {
    let recipe = use_context::<RwSignal<CreateRecipe>>().unwrap();

    let instruction = RwSignal::new("".to_string());

    view! {
        <FormGroup>
            <FormFieldTextarea
                name="instruction"
                class="col-span-12"
                value=instruction
                placeholder="Instruction"
            />
            <Button
                button_type=ButtonType::Button
                class="col-span-12"
                on:click=move |_| {
                    recipe
                        .update(|r| {
                            if let Some(ref mut instructions) = r.instructions {
                                instructions.push(instruction.get_untracked());
                            } else {
                                r.instructions = Some(vec![instruction.get_untracked()]);
                            }
                            instruction.set("".to_string());
                        })
                }
            >

                "Add to instructions"
            </Button>
            <ul class="col-span-12">
                // This is not so good since it rerenders the entire list on each change. However, it was a
                // bit tricky to find a good way to do it with `<For>`, since I want to be able to remove a
                // specific element, and the index is easy to do it. This works for now
                {move || {
                    let steps = recipe().instructions.unwrap_or_default();
                    steps
                        .into_iter()
                        .enumerate()
                        .map(|(index, step)| {
                            view! { <RecipeStepCard index=index step=step recipe=recipe /> }
                        })
                        .collect::<Vec<_>>()
                }}

            </ul>
        </FormGroup>
    }
}

#[component]
fn RecipeStepCard(index: usize, step: String, recipe: RwSignal<CreateRecipe>) -> impl IntoView {
    let num_steps = move || recipe().instructions.unwrap_or_default().len();
    let remove_card = move |index: usize| {
        recipe.update(|r| {
            let instructions = r.instructions.as_mut().unwrap();
            if instructions.len() == 1 {
                r.instructions = None;
            } else {
                instructions.remove(index);
            }
        })
    };

    let swap_card = move |index: usize, other: usize| {
        recipe.update(|r| {
            r.instructions.as_mut().unwrap().swap(index, other);
        })
    };

    view! {
        <li class="col-span-12">
            <Card>
                <CardHeader>
                    <h1>Step {index + 1}</h1>
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
                {step}
            </Card>
        </li>
    }
}
