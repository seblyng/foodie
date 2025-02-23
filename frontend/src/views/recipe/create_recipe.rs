// use crate::views::recipe::recipe_form::try_upload_image;
use leptos::{prelude::*, task::spawn_local};
use leptos_router::{hooks::use_navigate, NavigateOptions};
use std::time::Duration;

use crate::{
    // components::stepper::Stepper,
    context::toast::{use_toast, Toast, ToastType, ToasterTrait},
    request::post,
};

use common::recipe::{CreateRecipe, Recipe};
// use web_sys::File;

use crate::components::form::Form;

#[component]
pub fn CreateRecipe() -> impl IntoView {
    let recipe = RwSignal::new(common::recipe::CreateRecipe::default());
    let toast = use_toast().unwrap();

    // let file = create_rw_signal::<Option<File>>(None);

    let on_submit = move |create_recipe: CreateRecipe| {
        spawn_local(async move {
            // if let Ok(Some(img)) = try_upload_image(file.get_untracked()).await {
            //     create_recipe.img = Some(img);
            // }

            let body = serde_json::to_value(create_recipe).unwrap();
            let res = post("/api/recipe").body(body.to_string()).send().await;

            match res {
                Ok(r) if r.ok() => {
                    toast.add(Toast {
                        ty: ToastType::Success,
                        body: "Successfully uploaded recipe!".to_string(),
                        timeout: Some(Duration::from_secs(5)),
                    });

                    let navigate = use_navigate();
                    if let Ok(recipe) = r.json::<Recipe>().await {
                        navigate(
                            &format!("/recipes/{}", recipe.id),
                            NavigateOptions::default(),
                        );
                    } else {
                        navigate("/", NavigateOptions::default());
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

    // let (current_file, _) = signal::<Option<String>>(None);

    view! {
        <Form values=recipe on_submit=on_submit>
            // <Stepper>
            // <Step
            // label="Basics"
            // child=move || view! { <RecipeInfo current_file=current_file/> }
            // />
            // <Step label="Ingredients" child=move || view! { <RecipeIngredients/> }/>
            // <Step label="Steps" child=move || view! { <RecipeSteps/> }/>
            // </Stepper>

            // TODO: Have the save button on the final page
            <button type="submit" class="btn btn-primary">
                {"Save"}
            </button>
        </Form>
    }
}
