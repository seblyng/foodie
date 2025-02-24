use crate::components::{form::form_fields::get_span, input::Input};
use leptos::{prelude::*, tachys::html::property::IntoProperty};
use std::fmt::Display;

pub enum FormFieldInputType {
    Text,
    Number,
    Email,
    Password,
}

impl Display for FormFieldInputType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormFieldInputType::Text => write!(f, "text"),
            FormFieldInputType::Password => write!(f, "password"),
            FormFieldInputType::Email => write!(f, "email"),
            FormFieldInputType::Number => write!(f, "number"),
        }
    }
}

#[component]
pub fn FormFieldInput<T, U>(
    value: U,
    ty: FormFieldInputType,
    placeholder: &'static str,
    on_input: T,
    #[prop(optional)] span: &'static str,
) -> impl IntoView
where
    T: Fn(String) + 'static,
    U: IntoProperty + Clone + Send + Sync,
{
    let class = get_span(span);

    view! {
        <div class=class>
            <Input
                value=value
                placeholder=placeholder
                ty=ty.to_string()
                class="w-full"
                on:input=move |ev| {
                    on_input(event_target_value(&ev));
                }
            />

        </div>
    }
}
