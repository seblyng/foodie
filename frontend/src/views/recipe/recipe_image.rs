use leptos::prelude::*;
use thaw::*;

#[component]
pub fn RecipeImage(#[prop(optional, into)] src: MaybeProp<String>) -> impl IntoView {
    // A simple SVG placeholder (gray background, crossed lines)
    let placeholder = "data:image/svg+xml,%3Csvg width='400' height='300' xmlns='http://www.w3.org/2000/svg'%3E%3Crect width='400' height='300' fill='%23e5e7eb'/%3E%3Cline x1='0' y1='0' x2='400' y2='300' stroke='%239ca3af' stroke-width='8'/%3E%3Cline x1='400' y1='0' x2='0' y2='300' stroke='%239ca3af' stroke-width='8'/%3E%3C/svg%3E".to_string();
    let img_src = match src.get() {
        Some(src) if !src.is_empty() => src.clone(),
        _ => placeholder.clone(),
    };

    // Use a signal to swap to placeholder if image fails to load
    let (current_src, set_current_src) = signal(img_src);

    view! {
        <Image
            src=current_src
            alt="Recipe img"
            on:error=move |_| set_current_src.set(placeholder.clone())
        />
    }
}
