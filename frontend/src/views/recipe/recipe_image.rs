use leptos::prelude::*;

#[component]
pub fn RecipeImage(#[prop(optional, into)] src: Option<String>) -> impl IntoView {
    view! {
        <figure class="w-full">
            <img class="rounded-lg object-cover aspect-[4/3]" src=src alt="Recipe img"/>
        </figure>
    }
}
