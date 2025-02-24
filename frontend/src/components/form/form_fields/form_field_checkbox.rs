use leptos::prelude::*;

#[component]
pub fn FormFieldCheckbox<T>(placeholder: &'static str, on_checked: T) -> impl IntoView
where
    T: Fn(bool) + 'static,
{
    view! {
        <input
            type="checkbox"
            class="checkbox"
            placeholder=placeholder
            on:input=move |ev| {
                on_checked(event_target_checked(&ev));
            }
        />
    }
}
