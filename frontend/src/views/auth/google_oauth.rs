use leptos::{prelude::*, task::spawn_local};
use thaw::*;
use web_sys::window;

#[component]
pub fn Google() -> impl IntoView {
    let login = move |_| {
        spawn_local(async move {
            let res = reqwasm::http::Request::get("/api/oauth/google/login")
                .send()
                .await
                .unwrap();

            let url = res.text().await.unwrap();
            window().unwrap().location().set_href(&url).unwrap();
        });
    };
    view! {
        <Button button_type=ButtonType::Button appearance=ButtonAppearance::Primary on:click=login>
            <img
                class="w-6 h-6"
                src="https://www.svgrepo.com/show/475656/google-color.svg"
                loading="lazy"
                alt="google logo"
            />
            <span>Login with Google</span>
        </Button>
    }
}
