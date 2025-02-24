use leptos::prelude::*;
use web_sys::SubmitEvent;

pub mod form_fields;

#[component]
pub fn Form<T, U>(
    values: RwSignal<T, LocalStorage>,
    children: Children,
    on_submit: U,
) -> impl IntoView
where
    T: 'static + Clone,
    U: Fn(T) + 'static,
{
    provide_context(values);

    let internal_on_submit = move |e: SubmitEvent| {
        e.prevent_default();
        on_submit(values.get());
    };

    // TODO: Add on dirty to add save button or something
    view! {
        <div class="p-4 mb-4 w-full justify-center flex flex-col items-center">
            <form
                on:submit=internal_on_submit
                class="grid grid-auto-columns max-w-2xl w-full gap-4"
            >
                {children()}
            </form>
        </div>
    }
}

#[component]
pub fn FormGroup(children: Children) -> impl IntoView {
    view! { <div class="grid grid-cols-12 gap-4 justify-start">{children()}</div> }
}
