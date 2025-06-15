use std::time::Duration;

use common::user::User;
use leptos::{prelude::*, task::spawn_local};
use thaw::*;

use crate::{
    context::toast::{use_toast, Toast, ToastType, ToasterTrait},
    request::get,
};

#[component]
pub fn Friends() -> impl IntoView {
    let search = RwSignal::new(String::new());
    let toast = use_toast().unwrap();
    let users: RwSignal<Vec<User>> = RwSignal::new(vec![]);

    let on_click = move |_| {
        // TODO(seb): I should really add pagination for this
        spawn_local(async move {
            let s = search();
            let res = get(&format!("/api/users?search={s}")).send().await;

            let Ok(res) = res else {
                toast.add(Toast {
                    ty: ToastType::Error,
                    body: "Failed search for users".to_string(),
                    timeout: Some(Duration::from_secs(5)),
                });
                return;
            };

            let _users = res.json::<Vec<User>>().await.unwrap();
            users.set(_users);
        })
    };

    view! {
        <div>
            <Input value=search />
            <Button on_click=on_click>"Search"</Button>
            <ul>
                {move || {
                    users()
                        .into_iter()
                        .map(|u| {
                            view! {
                                <li>
                                    <UserList user=u />
                                </li>
                            }
                        })
                        .collect::<Vec<_>>()
                }}
            </ul>
        </div>
    }
}

#[component]
pub fn UserList(user: User) -> impl IntoView {
    view! {
        <Card>
            <CardHeader>
                <p>{user.name}</p>
                <CardHeaderAction slot>
                    <Button button_type=ButtonType::Button on:click=move |_| {}>
                        "Add friend"
                    </Button>
                </CardHeaderAction>
            </CardHeader>

        </Card>
    }
}
