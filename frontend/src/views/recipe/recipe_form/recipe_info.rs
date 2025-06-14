use leptos::prelude::*;
use std::time::Duration;
use thaw::*;
use web_sys::{File, Url};

use crate::components::form::form_fields::form_field_input::FormFieldInput;
use crate::components::form::form_fields::form_field_number_input::FormFieldNumberInput;
use crate::context::toast::{use_toast, Toast, ToastType, ToasterTrait};
use common::recipe::CreateRecipe;

use crate::components::form::FormGroup;
use crate::views::recipe::recipe_image::RecipeImage;

#[component]
pub fn RecipeInfo(
    file: (
        ReadSignal<Option<File>, LocalStorage>,
        WriteSignal<Option<File>, LocalStorage>,
    ),
    current_file: ReadSignal<Option<String>>,
) -> impl IntoView {
    let recipe = use_context::<RwSignal<CreateRecipe>>().unwrap();
    let name = slice!(recipe.name);
    let servings = RwSignal::new(0);

    Effect::new(move || {
        let serv = servings();
        recipe.update(|r| r.servings = serv);
    });

    let description = create_slice(
        recipe,
        |r| r.description.clone().unwrap_or_default(),
        |r, s: String| r.description = Some(s),
    );

    view! {
        <div>
            <FormGroup>
                <FileInput file=file current_file=current_file />
                <FormFieldInput
                    name="name"
                    placeholder="Name"
                    value=name
                    rules=vec![InputRule::required(true.into())]
                />

                <FormFieldNumberInput<
                i32,
            >
                    name="servings"
                    step_page=1
                    placeholder="Servings"
                    value=servings
                    rules=vec![
                        SpinButtonRule::validator(move |v: &i32, _| {
                            if (0..=72).contains(v) {
                                Ok(())
                            } else {
                                Err(
                                    FieldValidationState::Error(
                                        "Must be a number between 0 and 72".to_string(),
                                    ),
                                )
                            }
                        }),
                    ]
                />

                <RecipeDuration recipe />

                <Textarea class="col-span-12" value=description placeholder="Description" />
            </FormGroup>
        </div>
    }
}

#[component]
fn RecipeDuration(recipe: RwSignal<CreateRecipe>) -> impl IntoView {
    let baking_time_minutes = RwSignal::new(0);

    Effect::new(move || {
        let total_minutes = baking_time_minutes();
        let hours = total_minutes / 60;
        let minutes = total_minutes % 60;
        if let Some(baking_time) = chrono::NaiveTime::from_hms_opt(hours, minutes, 0) {
            recipe.update(|r| {
                r.baking_time = Some(baking_time);
            });
        }
    });

    let prep_time_minutes = RwSignal::new(0);

    Effect::new(move || {
        let total_minutes = prep_time_minutes();
        let hours = total_minutes / 60;
        let minutes = total_minutes % 60;
        if let Some(prep_time) = chrono::NaiveTime::from_hms_opt(hours, minutes, 0) {
            recipe.update(|r| {
                r.prep_time = Some(prep_time);
            });
        }
    });

    view! {
        <FormFieldNumberInput<
        u32,
    >
            name="baking_time"
            step_page=1
            placeholder="Baking time minutes"
            value=baking_time_minutes
            rules=vec![
                SpinButtonRule::validator(move |v: &u32, _| {
                    if (0..=500).contains(v) {
                        Ok(())
                    } else {
                        Err(
                            FieldValidationState::Error(
                                "Must be a number between 0 and 500".to_string(),
                            ),
                        )
                    }
                }),
            ]
        />

        <FormFieldNumberInput<
        u32,
    >
            name="prep_time"
            step_page=1
            placeholder="Prep time minutes"
            value=prep_time_minutes
            rules=vec![
                SpinButtonRule::validator(move |v: &u32, _| {
                    if (0..=500).contains(v) {
                        Ok(())
                    } else {
                        Err(
                            FieldValidationState::Error(
                                "Must be a number between 0 and 500".to_string(),
                            ),
                        )
                    }
                }),
            ]
        />
    }
}

#[component]
fn FileInput(
    file: (
        ReadSignal<Option<File>, LocalStorage>,
        WriteSignal<Option<File>, LocalStorage>,
    ),
    current_file: ReadSignal<Option<String>>,
) -> impl IntoView {
    let toast = use_toast().unwrap();

    let img = move || {
        let blob = file.0().unwrap().slice().unwrap();
        Url::create_object_url_with_blob(&blob).unwrap().to_string()
    };

    let image_view = move || {
        if file.0().is_some() {
            view! { <RecipeImage src=img() /> }.into_any()
        } else if current_file().is_some() {
            view! { <RecipeImage src=current_file().unwrap() /> }.into_any()
        } else {
            view! {
                <UploadDragger class="h-full">
                    <Icon icon=icondata::AiCloudUploadOutlined />
                    <p>"Upload image for your recipe"</p>
                </UploadDragger>
            }
            .into_any()
        }
    };

    let custom_request = move |files: FileList| {
        if files.length() > 1 {
            toast.add(Toast {
                ty: ToastType::Error,
                body: "Only allowed to upload 1 file".to_string(),
                timeout: Some(Duration::from_secs(5)),
            });
            return;
        }

        file.1.set(files.get(0));
    };

    view! {
        <div class="col-span-12">
            <Upload accept="image/*" custom_request class="flex flex-col">
                {image_view}
            </Upload>

        </div>
    }
}
