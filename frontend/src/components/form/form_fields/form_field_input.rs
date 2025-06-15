use leptos::prelude::*;
use thaw::*;
use thaw_utils::class_list;

#[component]
pub fn FormFieldInput(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] name: MaybeProp<String>,
    #[prop(optional, into)] value: thaw_utils::Model<String>,
    #[prop(optional, into)] placeholder: MaybeProp<String>,
    #[prop(optional, into)] input_type: Signal<InputType>,
    #[prop(optional, into)] rules: Vec<InputRule>,
) -> impl IntoView {
    view! {
        <div class=class_list![class, "col-span-12"]>
            <Field name=name>
                <Input
                    rules=rules
                    value=value
                    placeholder=placeholder
                    input_type=input_type
                    class="w-full"
                />
            </Field>
        </div>
    }
}
