use leptos::{prelude::*, task::spawn_local};
use leptos_router::hooks::use_navigate;
use thaw::*;

use crate::{context::auth::AuthContext, request::post};

#[component]
fn Profile() -> impl IntoView {
    let auth = use_context::<AuthContext>().unwrap().0;

    let navigate = use_navigate();

    view! {
        {move || {
            match auth.get() {
                Some(_auth) => {
                    let nav = navigate.clone();
                    let on_select = move |key: String| {
                        let nav = nav.clone();
                        if key == "logout" {
                            spawn_local(async move {
                                post("/api/logout").send().await.unwrap();
                                nav("/", Default::default());
                                auth.set(Some(None));
                            });
                        } else {
                            nav("/profile", Default::default());
                        }
                    };
                    if _auth.is_some() {
                        view! {
                            <Menu on_select position=MenuPosition::BottomEnd>
                                <MenuTrigger slot>
                                    <Avatar />
                                </MenuTrigger>
                                <MenuItem value="profile">"Profile"</MenuItem>
                                <MenuItem value="logout">"Logout"</MenuItem>
                            </Menu>
                        }
                            .into_any()
                    } else {
                        let nav = navigate.clone();
                        view! {
                            <Button
                                appearance=ButtonAppearance::Transparent
                                on_click=move |_| nav("/login", Default::default())
                            >
                                "Log in"
                            </Button>
                        }
                            .into_any()
                    }
                }
                None => {
                    let nav = navigate.clone();
                    view! {
                        <Button
                            appearance=ButtonAppearance::Transparent
                            on_click=move |_| nav("/login", Default::default())
                        >
                            "Log in"
                        </Button>
                    }
                        .into_any()
                }
            }
        }}
    }
}

#[component]
pub fn Navbar() -> impl IntoView {
    let navigate = use_navigate();
    let on_select = move |key: String| {
        let nav = navigate.clone();
        match key.as_str() {
            "home" => nav("/", Default::default()),
            "create_recipe" => nav("/recipes/create", Default::default()),
            "friends" => nav("/friends", Default::default()),
            _ => unreachable!("Should not happen"),
        }
    };

    view! {
        <LayoutHeader
            class="flex align-center justify-between top-0 fixed w-full z-1000 p-4"
            attr:style="background-color: var(--colorNeutralStroke1);"
        >
            <Flex>
                <Menu position=MenuPosition::BottomStart on_select=on_select.clone()>
                    <MenuTrigger slot>
                        <Button
                            appearance=ButtonAppearance::Transparent
                            icon=icondata::ChMenuHamburger
                        />
                    </MenuTrigger>
                    <MenuItem value="home">"Home"</MenuItem>
                    <MenuItem value="create_recipe">"Create recipe"</MenuItem>
                    <MenuItem value="friends">"Friends"</MenuItem>
                </Menu>

            </Flex>

            <a class="text-xl" href="/">
                "Foodie"
            </a>

            <Flex>
                <Profile />
            </Flex>
        </LayoutHeader>
    }
}
