use leptos::prelude::*;
use thaw::*;

#[component]
pub fn FormFieldNumberInput(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] name: MaybeProp<String>,
    #[prop(optional, into)] value: thaw_utils::Model<String>,
    #[prop(optional, into)] placeholder: MaybeProp<String>,
    #[prop(optional, into)] rules: Vec<InputRule>,
) -> impl IntoView {
    view! {
        <Field class=class name=name label=placeholder>
            <Input
                placeholder=placeholder
                class="w-full"
                value=value
                rules=rules
                allow_value=move |val: String| { val.parse::<f64>().is_ok() || val.is_empty() }
            />
        </Field>
    }
}
