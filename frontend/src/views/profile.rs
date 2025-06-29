use leptos::task::spawn_local;
use std::time::Duration;
use thaw::*;

use common::user::{User, UserWithRelation};
use leptos::prelude::*;
use leptos::prelude::{Get, Transition};

use crate::request::post;
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

                                <PendingRequests />
                            }
                                .into_any()
                        }
                    })
            }}

        </Transition>
    }
}

#[component]
pub fn PendingRequests() -> impl IntoView {
    let toast = use_toast().unwrap();

    let users_resource = LocalResource::new(move || async move {
        let res = match get("/api/friends/pending").send().await {
            Ok(res) => res,
            Err(_) => {
                toast.add(Toast {
                    ty: ToastType::Error,
                    body: "Couldn't get pending friend requests".to_string(),
                    timeout: Some(Duration::from_secs(5)),
                });
                return None;
            }
        };

        res.json::<Vec<UserWithRelation>>().await.ok()
    });

    let _profile = move || users_resource.get().as_deref().map(|it| it.to_owned());

    view! {
        <Transition fallback=Loading>
            {move || {
                _profile()
                    .map(|data| match data {
                        None => ().into_any(),
                        Some(users) => {
                            view! {
                                <ul>
                                    {users
                                        .into_iter()
                                        .map(|u| {
                                            view! {
                                                <li>
                                                    <UserList user=u />
                                                </li>
                                            }
                                        })
                                        .collect::<Vec<_>>()}
                                </ul>
                            }
                                .into_any()
                        }
                    })
            }}

        </Transition>
    }
}

enum Answer {
    Accept,
    Reject,
}

// TODO(seb): Remove them from the list after answering
#[component]
pub fn UserList(user: UserWithRelation) -> impl IntoView {
    let toast = use_toast().unwrap();
    let on_click = move |id: i32, answer: Answer| {
        spawn_local(async move {
            let url = match answer {
                Answer::Accept => format!("/api/friends/accept/{id}"),
                Answer::Reject => format!("/api/friends/reject/{id}"),
            };
            let res = post(&url).send().await;

            let Ok(res) = res else {
                toast.add(Toast {
                    ty: ToastType::Error,
                    body: "Failed to answer friend request".to_string(),
                    timeout: Some(Duration::from_secs(5)),
                });
                return;
            };

            if !res.ok() {
                toast.add(Toast {
                    ty: ToastType::Error,
                    body: "Failed to answer friend request".to_string(),
                    timeout: Some(Duration::from_secs(5)),
                });
                return;
            }

            toast.add(Toast {
                ty: ToastType::Success,
                body: "Answered friend request".to_string(),
                timeout: Some(Duration::from_secs(5)),
            });
        })
    };

    view! {
        <Card>
            <CardHeader>
                <p>{user.name}</p>
                <CardHeaderAction slot>
                    <Button
                        button_type=ButtonType::Button
                        on:click=move |_| on_click(user.id, Answer::Accept)
                    >
                        "Accept"
                    </Button>
                    <Button
                        button_type=ButtonType::Button
                        on:click=move |_| on_click(user.id, Answer::Reject)
                    >
                        "Reject"
                    </Button>
                </CardHeaderAction>
            </CardHeader>

        </Card>
    }
}
