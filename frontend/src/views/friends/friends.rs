use std::time::Duration;

use common::{friendship::FriendshipStatus, user::UserWithRelation};
use leptos::{prelude::*, task::spawn_local};
use thaw::*;

use crate::{
    context::toast::{use_toast, Toast, ToastType, ToasterTrait},
    request::{get, post},
};

#[component]
pub fn Friends() -> impl IntoView {
    let search = RwSignal::new(String::new());
    let toast = use_toast().unwrap();
    let users: RwSignal<Vec<UserWithRelation>> = RwSignal::new(vec![]);

    let on_click = move |_| {
        // TODO(seb): I should really add pagination for this
        let s = search();
        spawn_local(async move {
            let res = get(&format!("/api/users?search={s}")).send().await;

            let Ok(res) = res else {
                toast.add(Toast {
                    ty: ToastType::Error,
                    body: "Failed search for users".to_string(),
                    timeout: Some(Duration::from_secs(5)),
                });
                return;
            };

            let _users = res.json::<Vec<UserWithRelation>>().await.unwrap();
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
pub fn UserList(user: UserWithRelation) -> impl IntoView {
    let (status, set_status) = signal(user.status);
    let button_text = move || match status() {
        Some(s) => match s {
            FriendshipStatus::Pending => "Pending",
            FriendshipStatus::Accepted => "Accepted",
            FriendshipStatus::Rejected => "Rejected",
            FriendshipStatus::Blocked => "Blocked",
        },
        None => "Add friend",
    };

    let toast = use_toast().unwrap();
    let on_click = move |id: i32| {
        if status().is_some() {
            return;
        }

        spawn_local(async move {
            // Only allow to send a request if they have no status
            let res = post(&format!("/api/friends/new/{id}")).send().await;

            let Ok(res) = res else {
                toast.add(Toast {
                    ty: ToastType::Error,
                    body: "Failed to send friend request".to_string(),
                    timeout: Some(Duration::from_secs(5)),
                });
                return;
            };

            if !res.ok() {
                toast.add(Toast {
                    ty: ToastType::Error,
                    body: "Failed to send friend request".to_string(),
                    timeout: Some(Duration::from_secs(5)),
                });
                return;
            }

            set_status(Some(FriendshipStatus::Pending));
        })
    };

    view! {
        <Card>
            <CardHeader>
                <p>{user.name}</p>
                <CardHeaderAction slot>
                    <Button button_type=ButtonType::Button on:click=move |_| on_click(user.id)>
                        {move || button_text()}
                    </Button>
                </CardHeaderAction>
            </CardHeader>

        </Card>
    }
}
