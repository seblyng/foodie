use chrono::{NaiveTime, Timelike};
use leptos::prelude::*;

use crate::components::{
    dropdown::{DropDown, DropDownItem},
    form::form_fields::get_span,
};

#[component]
pub fn FormFieldDuration<T>(
    value: Signal<NaiveTime>,
    max_hours: usize,
    placeholder: &'static str,
    on_change: T,
    #[prop(optional)] span: &'static str,
) -> impl IntoView
where
    T: Fn(NaiveTime) + 'static + Copy,
{
    let (hours, set_hours) = signal(0);
    let (minutes, set_minutes) = signal(0);

    let f = |start: usize, end: usize| {
        (start..=end)
            .map(|i| DropDownItem {
                key: i,
                label: i,
                value: i,
            })
            .collect::<Vec<_>>()
    };

    let class = get_span(span);

    view! {
        <div class=class>
            <p>{placeholder}</p>
            <div class="grid grid-cols-2">
                <DropDown
                    value=Signal::derive(move || value().hour() as usize)
                    on_change=move |h| {
                        set_hours(h as u32);
                        on_change(chrono::NaiveTime::from_hms_opt(hours(), minutes(), 0).unwrap());
                    }

                    class="col-span-1 w-full".to_string()
                    placeholder="Hours"
                    items=f(0, max_hours)
                />
                <DropDown
                    value=Signal::derive(move || value().minute() as usize)
                    on_change=move |h| {
                        set_minutes(h as u32);
                        on_change(chrono::NaiveTime::from_hms_opt(hours(), minutes(), 0).unwrap());
                    }

                    class="col-span-1 w-full".to_string()
                    placeholder="Minutes"
                    items=f(0, 59)
                />
            </div>
        </div>
    }
}
