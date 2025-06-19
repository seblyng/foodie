use leptos::prelude::*;
use thaw::*;

#[component]
pub fn FormFieldSelect(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] name: MaybeProp<String>,
    #[prop(optional, into)] value: thaw_utils::Model<String>,
    #[prop(optional, into)] placeholder: MaybeProp<String>,
    #[prop(optional, into)] rules: Vec<ComboboxRule>,
    children: Children,
) -> impl IntoView {
    view! {
        <Field class=class name=name label=placeholder>
            <Combobox value=value placeholder=placeholder rules=rules clearable=true>
                {children()}
            </Combobox>
        </Field>
    }
}
