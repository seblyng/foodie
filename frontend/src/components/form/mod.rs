use leptos::prelude::*;
use thaw::{FieldContextInjection, FieldContextProvider};
use web_sys::SubmitEvent;

pub mod form_fields;

#[component]
pub fn Form<U>(children: Children, on_submit: U) -> impl IntoView
where
    U: Fn(SubmitEvent) + 'static + Send + Sync,
{
    view! {
        <div class="p-4 mb-4 w-full justify-center flex flex-col items-center">
            <FieldContextProvider>
                <form
                    on:submit={
                        let field_context = FieldContextInjection::expect_context();
                        move |e: SubmitEvent| {
                            e.prevent_default();
                            if field_context.validate() {
                                on_submit(e);
                            }
                        }
                    }
                    class="grid grid-auto-columns max-w-2xl w-full gap-4"
                >

                    {children()}
                </form>
            </FieldContextProvider>
        </div>
    }
}

#[component]
pub fn FormGroup(children: Children) -> impl IntoView {
    view! { <div class="grid grid-cols-12 gap-4 justify-start">{children()}</div> }
}
