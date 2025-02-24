use common::user::UserLogin;
use leptos::{prelude::*, task::spawn_local};
use leptos_router::{hooks::use_navigate, NavigateOptions};

use crate::{
    components::form::{
        form_fields::form_field_input::{FormFieldInput, FormFieldInputType},
        Form, FormGroup,
    },
    context::auth::AuthContext,
    views::auth::google_oauth::Google,
};

#[component]
pub fn Login() -> impl IntoView {
    let auth = use_context::<AuthContext>().unwrap().0;
    let user = RwSignal::new_with_storage(common::user::UserLogin::default());

    let on_submit = move |user: UserLogin| {
        let navigate = use_navigate();
        spawn_local(async move {
            let body = serde_json::to_value(user).unwrap();
            let res = reqwasm::http::Request::post("/api/login")
                .header("content-type", "application/json")
                .body(body.to_string())
                .send()
                .await
                .unwrap();

            if res.status() != 401 {
                navigate("/", NavigateOptions::default());
            }
        });
    };

    view! {
        <div class="flex justify-center h-navbar-screen">
            <Form values=user on_submit=on_submit>
                <FormGroup>
                    <FormFieldInput
                        value=move || user().email
                        on_input=move |email| user.update(|u| u.email = email)
                        placeholder="Email"
                        ty=FormFieldInputType::Email
                    />

                    <FormFieldInput
                        value=move || user().password
                        placeholder="Password"
                        ty=FormFieldInputType::Password
                        on_input=move |password| user.update(|u| u.password = password)
                    />
                </FormGroup>

                <button class="btn btn-primary" type="submit">
                    "Submit"
                </button>

                <Google/>
            </Form>
        </div>
    }
}
