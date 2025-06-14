use leptos::{prelude::*, task::spawn_local};
use leptos_router::{hooks::use_navigate, NavigateOptions};
use thaw::*;

use crate::{
    components::form::{FormGroup, NewForm},
    views::auth::google_oauth::Google,
};

#[component]
pub fn Login() -> impl IntoView {
    let navigate = use_navigate();
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());

    let on_submit_foo = move |_| {
        let nav = navigate.clone();
        spawn_local(async move {
            let user = common::user::UserLogin {
                email: email(),
                password: password(),
            };

            let body = serde_json::to_value(user).unwrap();
            let res = reqwasm::http::Request::post("/api/login")
                .header("content-type", "application/json")
                .body(body.to_string())
                .send()
                .await
                .unwrap();

            if res.status() != 401 {
                nav("/", NavigateOptions::default());
            }
        });
    };

    view! {
        <div class="flex justify-center h-navbar-screen">

            <NewForm on_submit=on_submit_foo>
                <FormGroup>
                    <div class="col-span-12">
                        <Input value=email placeholder="Email" class="w-full" />
                    </div>

                    <div class="col-span-12">
                        <Input
                            input_type=InputType::Password
                            value=password
                            placeholder="Password"
                            class="w-full"
                        />
                    </div>
                    <Button class="col-span-12" button_type=ButtonType::Submit>
                        "Submit"
                    </Button>

                </FormGroup>
                <Google />
            </NewForm>
        </div>
    }
}
