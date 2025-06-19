use std::ops::Add;
use std::time::Duration;

use chrono::{NaiveTime, Timelike};

pub mod create_recipe;
pub mod edit_recipe;
pub mod recipe;
pub mod recipe_form;
pub mod recipe_image;
pub mod recipes;

pub fn total_time(a: Option<NaiveTime>, b: Option<NaiveTime>) -> Option<NaiveTime> {
    match (a, b) {
        (Some(_a), Some(_b)) => {
            let duration = Duration::new(_b.num_seconds_from_midnight() as u64, _b.nanosecond());
            Some(_a.add(duration))
        }
        (Some(prep_time), None) => Some(prep_time),
        (None, Some(baking_time)) => Some(baking_time),
        (None, None) => None,
    }
}

pub fn format_time(time: NaiveTime) -> String {
    match (time.hour(), time.minute()) {
        (h, m) if h >= 1 && m >= 1 => format!("{h} h {m} min"),
        (h, _) if h >= 1 => format!("{h} h"),
        (_, m) if m >= 1 => format!("{m} min"),
        _ => "".to_string(),
    }
}

pub fn format_ingredients(len: usize) -> String {
    let val = if len > 1 { "ingredients" } else { "ingredient" };
    format!("{len} {val}")
}
