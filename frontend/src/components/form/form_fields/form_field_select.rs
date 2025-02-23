use leptos::prelude::*;

use crate::components::{
    dropdown::{DropDown, DropDownItem},
    form::form_fields::get_span,
};

#[component]
pub fn FormFieldSelect<T, V, I, S>(
    value: Signal<I>,
    items: Vec<DropDownItem<V, I, S>>,
    on_change: T,
    placeholder: &'static str,
    #[prop(optional)] span: &'static str,
) -> impl IntoView
where
    T: Fn(Option<V>) + 'static + Clone,
    V: Clone + 'static,
    I: Eq + PartialEq + Clone + std::hash::Hash + 'static,
    S: std::fmt::Display + Clone + 'static,
{
    let class = get_span(span);

    let internal_items = items.clone();
    let on_change = move |v| {
        let new_item = internal_items.iter().find(|i| i.key == v);
        on_change(new_item.map(|it| it.value.clone()));
    };

    view! {
        <DropDown class=class value=value on_change=on_change placeholder=placeholder items=items/>
    }
}
