use leptos::{html, prelude::*};
use std::time::Duration;
use web_sys::{File, Url};

use crate::{
    components::icons::file_upload_icon::FileUploadIcon,
    context::toast::{use_toast, Toast, ToastType, ToasterTrait},
};
use common::recipe::CreateRecipe;

use crate::components::{
    dropdown::DropDownItem,
    form::{
        form_fields::{
            form_field_duration::FormFieldDuration,
            form_field_input::{FormFieldInput, FormFieldInputType},
            form_field_select::FormFieldSelect,
            form_field_textarea::FormFieldTextarea,
        },
        FormGroup,
    },
};
use crate::views::recipe::recipe_image::RecipeImage;

#[component]
pub fn RecipeInfo(
    file: (
        ReadSignal<Option<File>, LocalStorage>,
        WriteSignal<Option<File>, LocalStorage>,
    ),
    current_file: ReadSignal<Option<String>>,
) -> impl IntoView {
    let items = (0..72)
        .map(|i| DropDownItem {
            key: i,
            value: i,
            label: i.to_string(),
        })
        .collect::<Vec<_>>();

    let recipe = use_context::<RwSignal<CreateRecipe, LocalStorage>>().unwrap();

    view! {
        <div class="card w-full bg-neutral">
            <div class="card-body">
                <h2 class="card-title">"General info about your recipe"</h2>
                <FormGroup>
                    <FileInput file=file current_file=current_file/>

                    <FormFieldInput
                        value=move || recipe().name
                        ty=FormFieldInputType::Text
                        placeholder="Name"
                        on_input=move |name| recipe.update(|r| r.name = name)
                    />

                    <FormFieldSelect
                        value=Signal::derive(move || recipe().servings)
                        items=items
                        placeholder="Servings"
                        on_change=move |servings| {
                            recipe.update(|r| r.servings = servings.unwrap_or_default())
                        }
                    />

                    <FormFieldDuration
                        value=Signal::derive(move || recipe().baking_time.unwrap_or_default())
                        max_hours=72
                        placeholder="Baking time"
                        on_change=move |baking_time| {
                            recipe.update(|r| r.baking_time = Some(baking_time))
                        }
                    />

                    <FormFieldDuration
                        value=Signal::derive(move || recipe().prep_time.unwrap_or_default())
                        max_hours=72
                        placeholder="Prep time"
                        on_change=move |prep_time| {
                            recipe.update(|r| r.prep_time = Some(prep_time))
                        }
                    />

                    <FormFieldTextarea
                        value=move || recipe().description
                        on_input=move |desc| recipe.update(|r| r.description = Some(desc))
                        placeholder="Description"
                    />
                </FormGroup>
            </div>
        </div>
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
    let file_input = NodeRef::<html::Input>::new();

    let on_change = move |_| {
        let Some(files) = file_input.get().unwrap().files() else {
            return;
        };

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

    let img = move || {
        let blob = file.0().unwrap().slice().unwrap();
        Url::create_object_url_with_blob(&blob).unwrap().to_string()
    };

    let image_view = move || {
        if file.0().is_some() {
            view! { <RecipeImage src=img()/> }.into_any()
        } else if current_file().is_some() {
            view! { <RecipeImage src=current_file().unwrap()/> }.into_any()
        } else {
            view! {
                <div class="flex flex-col items-center justify-center pt-5 pb-6">
                    <FileUploadIcon/>
                    <p class="mb-2 text-sm text-gray-500 dark:text-gray-400 font-semibold">
                        "Upload image for your recipe"
                    </p>
                </div>
            }
            .into_any()
        }
    };

    let style = move || {
        let mut class = "flex justify-center flex-col border-2 rounded-lg cursor-pointer bg-gray-700 border-gray-600 hover:bg-gray-600".to_string();
        if file.0.read().is_none() && current_file().is_none() {
            class.push_str(" min-h-96");
        }
        class
    };

    // TODO: Maybe not unwrap on slice?
    view! {
        <div class="col-span-12">
            <label for="dropzone-file" class=style>
                {image_view}
                <input
                    accept="image/*"
                    node_ref=file_input
                    id="dropzone-file"
                    type="file"
                    on:change=on_change
                    class="hidden"
                />
            </label>
        </div>
    }
}
