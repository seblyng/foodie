use leptos::prelude::*;

use components::navbar::Navbar;
use components::not_found::NotFound;

use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

use crate::components::custom_route::{private_route, public_route};
use crate::context::auth::AuthContext;
use crate::context::toast::Toaster;
use crate::views::auth::login_page::Login;
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
    provide_context(AuthContext::setup());

    mount_to_body(|| {
        view! {
            <Toaster>
                <Router>
                    <nav class="sticky top-0 z-[9999]">
                        <Navbar/>
                    </nav>
                    <main class="px-4 pt-4 pb-16 w-full">
                        <Routes fallback=|| NotFound>
                            <Route path=path!("/") view=public_route!(Home)/>
                            <Route path=path!("/login") view=public_route!(Login)/>
                            <Route path=path!("/profile") view=private_route!(Profile)/>
                            <Route path=path!("/recipes/:id") view=private_route!(Recipe)/>
                            <Route path=path!("/recipes/:id/edit") view=private_route!(EditRecipe)/>
                            <Route path=path!("/recipes") view=private_route!(Recipes)/>
                            <Route path=path!("/recipes/create") view=private_route!(CreateRecipe)/>
                        </Routes>
                    </main>
                </Router>
            </Toaster>
        }
    })
}
