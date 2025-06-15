use leptos::{prelude::*, task::spawn_local};
use leptos_router::{hooks::use_navigate, NavigateOptions};
use thaw::*;

use crate::{
    components::form::{form_fields::form_field_input::FormFieldInput, Form, FormGroup},
    views::auth::google_oauth::Google,
};

#[component]
pub fn Login() -> impl IntoView {
    let navigate = use_navigate();
    let user = RwSignal::new(common::user::UserLogin::default());
    let email = slice!(user.email);
    let password = slice!(user.password);

    let on_submit_foo = move |_| {
        let nav = navigate.clone();
        spawn_local(async move {
            let body = serde_json::to_value(user.get_untracked()).unwrap();
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
            <Form on_submit=on_submit_foo>
                <FormGroup>
                    <FormFieldInput
                        class="col-span-12"
                        name="email"
                        rules=vec![InputRule::required(true.into())]
                        value=email
                        placeholder="Email"
                    />

                    <FormFieldInput
                        class="col-span-12"
                        name="password"
                        input_type=InputType::Password
                        rules=vec![InputRule::required(true.into())]
                        value=password
                        placeholder="Password"
                    />

                    <Button class="col-span-12" button_type=ButtonType::Submit>
                        "Submit"
                    </Button>

                </FormGroup>
                <Google />
            </Form>
        </div>
    }
}
