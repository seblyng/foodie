use leptos::prelude::*;

#[component]
pub fn ModifyIcon() -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            class="stroke-current shrink-0 h-6 w-6"
            viewBox="0 0 24 24"
            fill="none"
        >
            <polygon points="16 3 21 8 8 21 3 21 3 16 16 3"></polygon>
        </svg>
    }
}
