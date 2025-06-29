use leptos::prelude::*;
use leptos_router::{components::Redirect, hooks::use_location};

use crate::{
    components::loading::Loading,
    context::auth::{AuthContext, AuthStore},
};

macro_rules! public_route {
    ($component:tt) => {
        || {
            use $crate::components::custom_route::PublicRoute;
            view! {
                <PublicRoute>
                    <$component/>
                </PublicRoute>
            }
        }
    };
}
macro_rules! private_route {
    ($component:tt) => {
        || {
            use $crate::components::custom_route::PrivateRoute;
            view! {
                <PrivateRoute>
                    <$component/>
                </PrivateRoute>
            }
        }
    };
}

pub(crate) use private_route;
pub(crate) use public_route;

#[component]
pub fn PrivateRoute(children: ChildrenFn) -> impl IntoView {
    let auth = use_context::<AuthContext>().unwrap().0;

    view! {
        {move || {
            match auth.get() {
                Some(auth) => {
                    if let Some(auth) = auth {
                        let store = AuthStore {
                            id: auth.id,
                            name: auth.name,
                            email: auth.email,
                        };
                        provide_context(store);
                        children().into_any()
                    } else {

                        view! { <Redirect path="/login" /> }
                            .into_any()
                    }
                }
                None => view! { <Loading /> }.into_any(),
            }
        }}
    }
}

#[component]
pub fn PublicRoute(children: ChildrenFn) -> impl IntoView {
    let auth = use_context::<AuthContext>().unwrap().0;
    let location = use_location();

    view! {
        {move || {
            match auth.get() {
                Some(auth) => {
                    if let Some(auth) = auth {
                        let store = AuthStore {
                            id: auth.id,
                            name: auth.name,
                            email: auth.email,
                        };
                        provide_context(store);
                        if location.pathname.get() == "/login" {
                            return view! { <Redirect path="/" /> }.into_any();
                        }
                        children().into_any()
                    } else {
                        children().into_any()
                    }
                }
                None => {

                    view! { <Loading /> }
                        .into_any()
                }
            }
        }}
    }
}
