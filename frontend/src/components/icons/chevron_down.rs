use leptos::prelude::*;

#[component]
pub fn ChevronDown() -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            class="stroke-current shrink-0 h-6 w-6"
            viewBox="0 0 24 24"
            fill="none"
        >
            <path d="M6 9l6 6 6-6"></path>
        </svg>
    }
}
