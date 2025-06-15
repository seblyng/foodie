use std::time::Duration;

use common::user::User;
use leptos::prelude::*;
use leptos::prelude::{Get, Transition};

use crate::{
    components::{loading::Loading, not_found::NotFound},
    context::toast::{use_toast, Toast, ToastType, ToasterTrait},
    request::get,
};

#[component]
pub fn Profile() -> impl IntoView {
    let toast = use_toast().unwrap();

    let user = LocalResource::new(move || async move {
        let res = match get("/api/me").send().await {
            Ok(res) => res,
            Err(_) => {
                toast.add(Toast {
                    ty: ToastType::Error,
                    body: "Couldn't fetch user".to_string(),
                    timeout: Some(Duration::from_secs(5)),
                });
                return None;
            }
        };

        res.json::<User>().await.ok()
    });

    let _profile = move || user.get().as_deref().map(|it| it.to_owned());

    view! {
        <Transition fallback=Loading>
            {move || {
                _profile()
                    .map(|data| match data {
                        None => NotFound.into_any(),
                        Some(user) => {
                            view! {
                                <p>{user.name}</p>
                                <p>{user.email}</p>
                            }
                                .into_any()
                        }
                    })
            }}

        </Transition>
    }
}
