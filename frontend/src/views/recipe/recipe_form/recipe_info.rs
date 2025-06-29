use chrono::Timelike;
use common::strum::IntoEnumIterator;
use leptos::prelude::*;
use std::time::Duration;
use thaw::*;
use web_sys::{File, Url};

use crate::components::form::form_fields::form_field_combobox::FormFieldSelect;
use crate::components::form::form_fields::form_field_input::FormFieldInput;
use crate::components::form::form_fields::form_field_number_input::FormFieldNumberInput;
use crate::components::form::form_fields::form_field_textarea::FormFieldTextarea;
use crate::context::toast::{use_toast, Toast, ToastType, ToasterTrait};
use common::recipe::{CreateRecipe, RecipeVisibility};

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
    let servings = RwSignal::new(recipe.get_untracked().servings.to_string());

    Effect::new(move || {
        let serv = servings().parse::<i32>().unwrap_or_default();
        recipe.update(|r| r.servings = serv);
    });

    let description = create_slice(
        recipe,
        |r| r.description.clone().unwrap_or_default(),
        |r, s: String| r.description = Some(s),
    );

    let visibility = create_slice(
        recipe,
        |r| r.visibility.to_string(),
        |r, n: String| {
            r.visibility = n
                .parse::<RecipeVisibility>()
                .unwrap_or(RecipeVisibility::Friends)
        },
    );

    view! {
        <FormGroup>
            <FileInput file=file current_file=current_file />
            <FormFieldInput
                name="name"
                class="col-span-12"
                placeholder="Name"
                value=name
                rules=vec![InputRule::required(true.into())]
            />

            <FormFieldNumberInput
                class="col-span-12"
                name="servings"
                placeholder="Servings"
                value=servings
                rules=vec![
                    InputRule::validator(move |v: &String, _| {
                        let Ok(val) = v.parse::<i32>() else {
                            return Err(
                                FieldValidationState::Error(
                                    "Must be a number between 0 and 72".to_string(),
                                ),
                            )
                        };
                        if (0..=72).contains(&val) {
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

            <FormFieldTextarea
                name="description"
                class="col-span-12"
                value=description
                placeholder="Description"
            />

            <FormFieldSelect class="col-span-12" value=visibility placeholder="Visibility">
                {move || {
                    common::recipe::RecipeVisibility::iter()
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
        </FormGroup>
    }
}

#[component]
fn RecipeDuration(recipe: RwSignal<CreateRecipe>) -> impl IntoView {
    let baking_time_minutes = RwSignal::new(
        recipe
            .get_untracked()
            .baking_time
            .map(|it| (it.hour() * 60 + it.minute()).to_string())
            .unwrap_or_default(),
    );

    Effect::new(move || {
        let Ok(total_minutes) = baking_time_minutes().parse::<u32>() else {
            return;
        };
        let hours = total_minutes / 60;
        let minutes = total_minutes % 60;
        if let Some(baking_time) = chrono::NaiveTime::from_hms_opt(hours, minutes, 0) {
            recipe.update(|r| {
                r.baking_time = Some(baking_time);
            });
        }
    });

    let prep_time_minutes = RwSignal::new(
        recipe
            .get_untracked()
            .prep_time
            .map(|it| (it.hour() * 60 + it.minute()).to_string())
            .unwrap_or_default(),
    );

    Effect::new(move || {
        let Ok(total_minutes) = prep_time_minutes().parse::<u32>() else {
            return;
        };
        let hours = total_minutes / 60;
        let minutes = total_minutes % 60;
        if let Some(prep_time) = chrono::NaiveTime::from_hms_opt(hours, minutes, 0) {
            recipe.update(|r| {
                r.prep_time = Some(prep_time);
            });
        }
    });

    let valid_time = |v: &String| {
        if v.is_empty() {
            return Ok(());
        }

        let Ok(val) = v.parse::<u32>() else {
            return Err(FieldValidationState::Error(
                "Must be a number between 0 and 500".to_string(),
            ));
        };
        match (0..500).contains(&val) {
            true => Ok(()),
            false => Err(FieldValidationState::Error(
                "Must be a number between 0 and 500".to_string(),
            )),
        }
    };

    view! {
        <FormFieldNumberInput
            class="col-span-12"
            name="baking_time"
            placeholder="Baking time"
            value=baking_time_minutes
            rules=vec![InputRule::validator(move |v: &String, _| valid_time(v))]
        />

        <FormFieldNumberInput
            class="col-span-12"
            name="prep_time"
            placeholder="Prep time"
            value=prep_time_minutes
            rules=vec![InputRule::validator(move |v: &String, _| valid_time(v))]
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
                <UploadDragger class="flex min-h-96 items-center justify-center">
                    <div class="flex items-center justify-center">
                        <Icon icon=icondata::AiCloudUploadOutlined />
                        <p>"Upload image for your recipe"</p>
                    </div>
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
