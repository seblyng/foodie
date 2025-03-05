use crate::components::stepper::{Step, Stepper};
use crate::views::recipe::recipe_form::recipe_info::RecipeInfo;
use crate::views::recipe::recipe_form::recipe_ingredients::RecipeIngredients;
use crate::views::recipe::recipe_form::recipe_steps::RecipeSteps;
use crate::views::recipe::recipe_form::try_upload_image;
use std::time::Duration;

use common::recipe::{CreateRecipe, Recipe};
use leptos::prelude::*;
use leptos::prelude::{RwSignal, With};
use leptos::task::spawn_local;
use leptos_router::hooks::use_params_map;
use web_sys::File;

use crate::{
    components::{form::Form, loading::Loading, not_found::NotFound},
    context::toast::{use_toast, Toast, ToastType, ToasterTrait},
    request::{get, put},
};

#[component]
pub fn EditRecipe() -> impl IntoView {
    let params = use_params_map();
    let toast = use_toast().unwrap();
    let id = move || params.with(|params| params.get("id").unwrap_or_default());

    let file = signal_local::<Option<File>>(None);
    let (current_file, set_current_file) = signal::<Option<String>>(None);

    let recipe = LocalResource::new(move || async move {
        let r = get(&format!("/api/recipe/{}", id()))
            .send()
            .await
            .ok()?
            .json::<Recipe>()
            .await
            .ok()?;

        set_current_file(r.img.clone());

        Some(RwSignal::new_with_storage(CreateRecipe::from(r)))
    });

    let _recipe = move || recipe.get().as_deref().map(|it| it.to_owned());

    let on_submit = move |mut submit_data: CreateRecipe| {
        let _id = id();
        spawn_local(async move {
            // TODO: Tries to upload the image if there is one. See if I want to only
            // call this when I have an image, and not with `Option<File>`
            if let Ok(Some(img)) = try_upload_image(file.0.get_untracked()).await {
                submit_data.img = Some(img);
            }

            let body = serde_json::to_value(submit_data).unwrap();
            let res = put(&format!("/api/recipe/{}", _id))
                .body(body.to_string())
                .send()
                .await;

            match res {
                Ok(r) if r.ok() => {
                    toast.add(Toast {
                        ty: ToastType::Success,
                        body: "Successfully edited recipe!".to_string(),
                        timeout: Some(Duration::from_secs(5)),
                    });
                }
                _ => {
                    toast.add(Toast {
                        ty: ToastType::Error,
                        body: "Failed to edit recipe".to_string(),
                        timeout: Some(Duration::from_secs(5)),
                    });
                }
            };
        })
    };

    view! {
        <Transition fallback=Loading>
            {move || {
                _recipe()
                    .map(|data| match data {
                        None => NotFound.into_any(),
                        Some(r) => {
                            view! {
                                <Form values=r on_submit=on_submit>
                                    <Stepper>
                                        <Step
                                            label="Basics"
                                            child=move || {
                                                view! { <RecipeInfo file=file current_file=current_file/> }
                                            }
                                        />

                                        <Step
                                            label="Ingredients"
                                            child=move || view! { <RecipeIngredients/> }
                                        />
                                        <Step label="Steps" child=move || view! { <RecipeSteps/> }/>

                                    // TODO: Have the save button on the final page
                                    </Stepper>
                                    <button type="submit" class="btn btn-primary">
                                        {"Save"}
                                    </button>
                                </Form>
                            }
                                .into_any()
                        }
                    })
            }}

        </Transition>
    }
}
