use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::wasm_bindgen;

use crate::components::input::Input;

#[component]
pub fn DropDown<T, U, V, F>(
    items: Vec<DropDownItem<T, U, V>>,
    value: Signal<U>,
    on_change: F,
    #[prop(optional)] class: String,
    #[prop(optional)] placeholder: &'static str,
) -> impl IntoView
where
    T: Clone + 'static,
    U: Eq + PartialEq + Clone + std::hash::Hash + 'static + Sync + Send,
    V: std::fmt::Display + Clone + 'static,
    F: Fn(U) + 'static + Clone,
{
    let internal_items = items.clone();

    let value = move || {
        items
            .iter()
            .find(|it| it.key == value())
            .map(|it| it.label.to_string())
            .unwrap_or_default()
    };

    view! {
        <div class:dropdown class:select-bordered class=class>
            <Input class="w-full" value=value readonly=true placeholder=placeholder/>
            <ul
                tabindex="0"
                class="w-full dropdown-content z-[1] menu p-2 bg-base-200 rounded-box h-52 flex-nowrap overflow-y-scroll"
            >

                {internal_items
                    .into_iter()
                    .map(|item| {
                        let _on_change = on_change.clone();
                        view! {
                            <li on:click=move |_| {
                                _on_change(item.key.clone());
                                document()
                                    .active_element()
                                    .and_then(|el| el.dyn_into::<web_sys::HtmlElement>().ok())
                                    .and_then(|el| el.blur().ok());
                            }>

                                <a>{item.label.to_string()}</a>
                            </li>
                        }
                    })
                    .collect::<Vec<_>>()}
            </ul>
        </div>
    }
}

#[derive(Clone)]
pub struct DropDownItem<T, U, V>
where
    T: Clone,
    U: Eq + PartialEq + Clone + std::hash::Hash + Sync + Send,
    V: std::fmt::Display + Clone,
{
    pub key: U,
    pub label: V,
    pub value: T,
}

#[derive(Clone)]
struct InternalDropDownItem<T, U, V>
where
    T: Clone,
    U: Eq + PartialEq + Clone + std::hash::Hash,
    V: std::fmt::Display + Clone,
{
    key: U,
    label: V,
    value: T,
}

impl<T, U, V> From<InternalDropDownItem<T, U, V>> for DropDownItem<T, U, V>
where
    T: Clone,
    U: Eq + PartialEq + Clone + std::hash::Hash,
    V: std::fmt::Display + Clone,
{
    fn from(value: InternalDropDownItem<T, U, V>) -> Self {
        Self {
            key: value.key,
            label: value.label,
            value: value.value,
        }
    }
}
