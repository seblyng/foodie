use std::{
    ops::{Add, Sub},
    str::FromStr,
};

use leptos::prelude::*;
use num::Bounded;
use thaw::*;

#[component]
pub fn FormFieldNumberInput<T>(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] name: MaybeProp<String>,
    #[prop(optional, into)] value: thaw_utils::Model<T>,
    #[prop(optional, into)] placeholder: MaybeProp<String>,
    #[prop(optional, into)] rules: Vec<SpinButtonRule<T>>,
    #[prop(into)] step_page: Signal<T>,
) -> impl IntoView
where
    T: Send + Sync + std::fmt::Debug,
    T: Add<Output = T> + Sub<Output = T> + PartialOrd + Bounded,
    T: Default + Clone + FromStr + ToString + 'static,
{
    view! {
        <Field class=class name=name label=placeholder>
            <SpinButton<T>
                placeholder=placeholder
                class="w-full"
                step_page=step_page
                value=value
                rules=rules
            />
        </Field>
    }
}
