use leptos::{prelude::*, tachys::html::property::IntoProperty};

#[component]
pub fn Input<T>(
    value: T,
    #[prop(optional)] class: &'static str,
    #[prop(optional, into)] ty: String,
    #[prop(optional)] placeholder: &'static str,
    #[prop(optional, into)] readonly: Option<bool>,
) -> impl IntoView
where
    T: IntoProperty + Send + Sync,
{
    let id = uuid::Uuid::new_v4();

    view! {
        <div class="relative">
            <input
                id=id.to_string()
                prop:value=value
                placeholder=placeholder
                type=ty
                class=class
                readonly=readonly
                class:floating-label-input
                class:peer
            />
            <label for=id.to_string() class="floating-label">
                {placeholder}
            </label>
        </div>
    }
}
