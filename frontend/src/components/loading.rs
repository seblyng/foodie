use leptos::prelude::*;

#[component]
pub fn Loading() -> impl IntoView {
    view! {
        <div class="absolute right-1/2 bottom-1/2 transform translate-x-1/2 translate-y-1/2 ">
            <span class="loading loading-spinner loading-lg"></span>
        </div>
    }
}
