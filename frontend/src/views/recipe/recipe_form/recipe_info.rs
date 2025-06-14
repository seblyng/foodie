use chrono::Timelike;
use leptos::prelude::*;
use std::time::Duration;
use thaw::*;
use web_sys::{File, Url};

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
    let servings = create_slice(
        recipe,
        |r| r.servings.to_string(),
        |r, s: String| r.servings = s.parse::<i32>().unwrap_or_default(),
    );

    let fi = |start: usize, end: usize| {
        (start..=end)
            .map(|i| {
                view! { <ComboboxOption value=i.to_string() text=i.to_string() /> }
            })
            .collect::<Vec<_>>()
    };

    let description = create_slice(
        recipe,
        |r| r.description.clone().unwrap_or_default(),
        |r, s: String| r.description = Some(s),
    );

    view! {
        <div>
            <FormGroup>
                <FileInput file=file current_file=current_file />
                <div class="col-span-12">
                    <Input class="w-full" value=name placeholder="Name" />
                </div>

                <Combobox class="col-span-12" value=servings placeholder="Servings">
                    {move || fi(0, 72)}
                </Combobox>

                <RecipeDuration recipe />

                <Textarea class="col-span-12" value=description placeholder="Description" />
            </FormGroup>
        </div>
    }
}

#[component]
fn RecipeDuration(recipe: RwSignal<CreateRecipe>) -> impl IntoView {
    let baking_time_minutes = create_slice(
        recipe,
        |r| {
            r.baking_time
                .map(|it| it.minute())
                .unwrap_or_default()
                .to_string()
        },
        move |r, s: String| {
            let total_minutes = s.parse::<u32>().unwrap();
            let hours = total_minutes / 60;
            let minutes = total_minutes % 60;
            r.baking_time = Some(chrono::NaiveTime::from_hms_opt(hours, minutes, 0).unwrap());
        },
    );

    let prep_time_minutes = create_slice(
        recipe,
        |r| {
            r.prep_time
                .map(|it| it.minute())
                .unwrap_or_default()
                .to_string()
        },
        move |r, s: String| {
            let total_minutes = s.parse::<u32>().unwrap();
            let hours = total_minutes / 60;
            let minutes = total_minutes % 60;
            r.prep_time = Some(chrono::NaiveTime::from_hms_opt(hours, minutes, 0).unwrap());
        },
    );

    let fi = |start: usize, end: usize| {
        (start..=end)
            .map(|i| {
                view! { <ComboboxOption value=i.to_string() text=i.to_string() /> }
            })
            .collect::<Vec<_>>()
    };

    view! {
        <Combobox class="col-span-6 md:col-span-3" value=baking_time_minutes placeholder="Minutes">
            {move || fi(0, 500)}
        </Combobox>

        <Combobox class="col-span-6 md:col-span-3" value=prep_time_minutes placeholder="Minutes">
            {move || fi(0, 500)}
        </Combobox>
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
