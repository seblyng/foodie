use leptos::{prelude::*, task::spawn_local};
use thaw::*;
use web_sys::window;

use crate::{
    components::form::{form_fields::form_field_input::FormFieldInput, Form, FormGroup},
    views::auth::google_oauth::Google,
};

#[component]
pub fn Login() -> impl IntoView {
    let user = RwSignal::new(common::user::UserLogin::default());
    let email = slice!(user.email);
    let password = slice!(user.password);

    let on_submit = move |_| {
        spawn_local(async move {
            let body = serde_json::to_value(user.get_untracked()).unwrap();
            let res = reqwasm::http::Request::post("/api/login")
                .header("content-type", "application/json")
                .body(body.to_string())
                .send()
                .await
                .unwrap();

            if res.status() != 401 {
                window().unwrap().location().set_href("/").unwrap();
            }
        });
    };

    view! {
        <div class="flex justify-center h-navbar-screen">
            <Form on_submit=on_submit>
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
