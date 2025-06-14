use leptos::prelude::*;
use thaw::FieldContextProvider;
use web_sys::SubmitEvent;

#[component]
pub fn NewForm<U>(children: Children, on_submit: U) -> impl IntoView
where
    U: Fn(SubmitEvent) + 'static,
{
    let internal_on_submit = move |e: SubmitEvent| {
        e.prevent_default();
        on_submit(e);
    };

    view! {
        <div class="p-4 mb-4 w-full justify-center flex flex-col items-center">
            <form
                on:submit=internal_on_submit
                class="grid grid-auto-columns max-w-2xl w-full gap-4"
            >
                <FieldContextProvider>{children()}</FieldContextProvider>
            </form>
        </div>
    }
}

#[component]
pub fn FormGroup(children: Children) -> impl IntoView {
    view! { <div class="grid grid-cols-12 gap-4 justify-start">{children()}</div> }
}
