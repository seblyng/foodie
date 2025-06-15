use leptos::prelude::*;
use thaw::*;

#[component]
pub fn FormFieldTextarea(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] name: MaybeProp<String>,
    #[prop(optional, into)] value: thaw_utils::Model<String>,
    #[prop(optional, into)] placeholder: MaybeProp<String>,
    #[prop(optional, into)] rules: Vec<TextareaRule>,
) -> impl IntoView {
    view! {
        <Field class=class name=name label=placeholder>
            <Textarea rules=rules value=value placeholder=placeholder class="w-full" />
        </Field>
    }
}
