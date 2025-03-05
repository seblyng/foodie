use leptos::prelude::*;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::context::toast::{use_toast, Toast, ToastType, ToasterTrait};

#[derive(Deserialize, Serialize)]
struct RecipeImage {
    name: String,
}

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div>
            <ToastTest/>
        </div>
    }
}

#[component]
fn ToastTest() -> impl IntoView {
    let toast = use_toast().unwrap();

    let error_toast = move |_| {
        toast.add(Toast {
            ty: ToastType::Error,
            body: "Error message".to_string(),
            timeout: Some(Duration::from_secs(3)),
        });
    };

    let warning_toast = move |_| {
        toast.add(Toast {
            ty: ToastType::Warning,
            body: "Warning message".to_string(),
            timeout: Some(Duration::from_secs(2)),
        })
    };

    let success_toast = move |_| {
        toast.add(Toast {
            ty: ToastType::Success,
            body: "Success message".to_string(),
            timeout: Some(Duration::from_secs(1)),
        })
    };

    view! {
        <div>
            <button class="btn btn-primary" on:click=error_toast>
                Add error toast
            </button>
            <button class="btn btn-primary" on:click=warning_toast>
                Add warning toast
            </button>
            <button class="btn btn-primary" on:click=success_toast>
                Add success toast
            </button>
        </div>
    }
}
