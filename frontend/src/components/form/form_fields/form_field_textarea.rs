use leptos::prelude::*;

use crate::components::{form::form_fields::get_span, textarea::Textarea};

#[component]
pub fn FormFieldTextarea<T>(
    value: Signal<String>,
    placeholder: &'static str,
    on_input: T,
    #[prop(optional)] span: &'static str,
) -> impl IntoView
where
    T: Fn(String) + 'static,
{
    let class = get_span(span);
    view! {
        <div class=class>
            <Textarea
                value=value
                class="w-full"
                placeholder=placeholder
                on:input=move |ev| {
                    on_input(event_target_value(&ev));
                }
            />

        </div>
    }
}
