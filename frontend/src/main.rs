use leptos::prelude::*;

use components::not_found::NotFound;

use leptos_meta::Html;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;
use thaw::{ConfigProvider, Layout, Theme};

use crate::components::custom_route::{private_route, public_route};
use crate::components::navbar::Navbar;
use crate::context::auth::AuthContext;
use crate::context::toast::Toaster;
use crate::views::auth::login_page::Login;
use crate::views::friends::friends::Friends;
use crate::views::home::Home;
use crate::views::profile::Profile;
use crate::views::recipe::create_recipe::CreateRecipe;
use crate::views::recipe::edit_recipe::EditRecipe;
use crate::views::recipe::recipe::Recipe;
use crate::views::recipe::recipes::Recipes;

mod components;
mod context;
mod request;
mod views;

pub fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(App)
}

#[component]
pub fn App() -> impl IntoView {
    let theme = RwSignal::new(Theme::dark());
    let color = move || theme.get().color.color_neutral_background_1;

    provide_context(AuthContext::setup());
    provide_context(theme);

    view! {
        <ConfigProvider theme>
            <Toaster>
                <Layout position=thaw::LayoutPosition::Absolute>
                    {move || view! { <Html {..} style=format!("background-color: {}", color()) /> }}
                    <Router>
                        <Navbar />
                        <main class="px-4 pt-20 w-full">
                            <Routes fallback=|| NotFound>
                                <Route path=path!("/") view=public_route!(Home) />
                                <Route path=path!("/login") view=public_route!(Login) />
                                <Route path=path!("/profile") view=private_route!(Profile) />
                                <Route path=path!("/recipes") view=private_route!(Recipes) />
                                <Route
                                    path=path!("/recipes/create")
                                    view=private_route!(CreateRecipe)
                                />
                                <Route path=path!("/recipes/:id") view=private_route!(Recipe) />
                                <Route
                                    path=path!("/recipes/:id/edit")
                                    view=private_route!(EditRecipe)
                                />
                                <Route
                                    path=path!("/recipes/:id/edit")
                                    view=private_route!(EditRecipe)
                                />

                                <Route path=path!("/friends") view=private_route!(Friends) />
                            </Routes>
                        </main>
                    </Router>
                </Layout>
            </Toaster>
        </ConfigProvider>
    }
}
