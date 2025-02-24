use leptos::{
    prelude::*,
    tachys::html::property::{IntoProperty, Property},
};

#[component]
pub fn Textarea<T>(
    value: T,
    #[prop(optional)] class: &'static str,
    #[prop(optional)] placeholder: &'static str,
) -> impl IntoView
where
    T: IntoProperty + 'static + Copy,
{
    let id = uuid::Uuid::new_v4();

    // let inner_value = move || {
    //     let v = match value.into_property() {
    //         Property::Value(v) => v,
    //         Property::Fn(f) => f(),
    //     };
    //     v.as_string().unwrap_or_default()
    // };

    view! {
        <div class="relative">
            <textarea
                id=id.to_string()
                placeholder=placeholder
                class=class
                class:floating-label-textarea
                class:peer
            >
                {"foo"}
            </textarea>
            <label for=id.to_string() class="floating-label">
                {placeholder}
            </label>
        </div>
    }
}
