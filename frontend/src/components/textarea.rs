use leptos::prelude::*;

#[component]
pub fn Textarea(
    value: Signal<String>,
    #[prop(optional)] class: &'static str,
    #[prop(optional)] placeholder: &'static str,
) -> impl IntoView {
    let id = uuid::Uuid::new_v4();

    view! {
        <div class="relative">
            <textarea
                prop:value=move || value.get()
                id=id.to_string()
                placeholder=placeholder
                class=class
                class:floating-label-textarea
                class:peer
            >
                {value.get_untracked()}
            </textarea>
            <label for=id.to_string() class="floating-label">
                {placeholder}
            </label>
        </div>
    }
}
