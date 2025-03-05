use crate::components::form::form_fields::form_field_textarea::FormFieldTextarea;
use crate::components::form::FormGroup;
use crate::components::icons::{
    chevron_down::ChevronDown, chevron_up::ChevronUp, close_icon::CloseIcon,
    modify_icon::ModifyIcon,
};
use common::recipe::CreateRecipe;
use leptos::prelude::*;

#[component]
pub fn RecipeSteps() -> impl IntoView {
    let recipe = use_context::<RwSignal<CreateRecipe, LocalStorage>>().unwrap();

    let instruction = RwSignal::new("".to_string());

    let var_name = view! {
        <div class="card w-full bg-neutral">
            <div class="card-body">
                <h2 class="card-title">"Add steps to your recipe"</h2>
                <FormGroup>
                    <FormFieldTextarea
                        value=instruction
                        on_input=move |i| instruction.set(i)
                        placeholder="Instruction"
                    />
                    <button
                        type="button"
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

                        class="btn btn-primary col-span-12"
                    >
                        "Add to instructions"
                    </button>
                </FormGroup>
            </div>
        </div>

        <ul>
            // This is not so good since it rerenders the entire list on each change. However, it was a
            // bit tricky to find a good way to do it with `<For>`, since I want to be able to remove a
            // specific element, and the index is easy to do it. This works for now
            {move || {
                let steps = recipe().instructions.unwrap_or_default();
                steps
                    .into_iter()
                    .enumerate()
                    .map(|(index, step)| {
                        view! { <RecipeStepCard index=index step=step recipe=recipe/> }
                    })
                    .collect::<Vec<_>>()
            }}

        </ul>
    };
    var_name
}

#[component]
fn RecipeStepCard(
    index: usize,
    step: String,
    recipe: RwSignal<CreateRecipe, LocalStorage>,
) -> impl IntoView {
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
        <li>
            <div class="card w-full bg-neutral">
                <div class="card-body">
                    <div class="card-actions flex justify-between">
                        <div>
                            <h2 class="card-title">Step {index + 1}</h2>
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
                    {step}
                </div>
            </div>
        </li>
    }
}
