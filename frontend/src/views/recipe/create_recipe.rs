use crate::{
    components::form::Form,
    views::recipe::recipe_form::{
        recipe_info::RecipeInfo, recipe_ingredients::RecipeIngredients, recipe_steps::RecipeSteps,
        try_upload_image,
    },
};
use leptos::{prelude::*, task::spawn_local};
use leptos_router::{hooks::use_navigate, NavigateOptions};
use std::time::Duration;
use thaw::*;
use web_sys::File;

use crate::{
    context::toast::{use_toast, Toast, ToastType, ToasterTrait},
    request::post,
};

use common::recipe::Recipe;

#[component]
pub fn CreateRecipe() -> impl IntoView {
    let recipe = RwSignal::new(common::recipe::CreateRecipe::default());

    provide_context(recipe);

    let toast = use_toast().unwrap();

    let file = signal_local::<Option<File>>(None);

    let navigate = use_navigate();

    let on_submit = move |_| {
        let mut create_recipe = recipe.get();
        let nav = navigate.clone();
        spawn_local(async move {
            match try_upload_image(file.0.get_untracked()).await {
                Ok(Some(img)) => create_recipe.img = Some(img),
                Err(_) => {
                    toast.add(Toast {
                        ty: ToastType::Error,
                        body: "Failed to upload image".to_string(),
                        timeout: Some(Duration::from_secs(5)),
                    });
                }
                _ => (),
            }

            let body = serde_json::to_value(create_recipe).unwrap();
            let res = post("/api/recipes").body(body.to_string()).send().await;

            match res {
                Ok(r) if r.ok() => {
                    toast.add(Toast {
                        ty: ToastType::Success,
                        body: "Successfully uploaded recipe!".to_string(),
                        timeout: Some(Duration::from_secs(5)),
                    });

                    if let Ok(recipe) = r.json::<Recipe>().await {
                        nav(
                            &format!("/recipes/{}", recipe.id),
                            NavigateOptions::default(),
                        );
                    } else {
                        nav("/", NavigateOptions::default());
                    };
                }
                _ => {
                    toast.add(Toast {
                        ty: ToastType::Error,
                        body: "Failed to upload recipe".to_string(),
                        timeout: Some(Duration::from_secs(5)),
                    });
                }
            };
        })
    };

    let (current_file, _) = signal::<Option<String>>(None);

    view! {
        <Form on_submit=on_submit>
            <RecipeInfo file=file current_file=current_file />
            <Divider />
            <RecipeIngredients />
            <Divider />
            <RecipeSteps />

            <Button appearance=ButtonAppearance::Primary button_type=ButtonType::Submit>
                {"Save"}
            </Button>
        </Form>
    }
}
