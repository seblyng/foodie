use common::user::User;
use leptos::prelude::{AsyncDerived, LocalStorage};

use crate::request::get;

#[derive(Clone)]
pub struct AuthContext(pub AsyncDerived<Option<User>, LocalStorage>);

#[derive(Debug, Clone)]
pub struct AuthStore {
    pub id: i32,
    pub name: String,
    pub email: String,
}

impl AuthContext {
    pub fn setup() -> Self {
        let me = AsyncDerived::new_unsync(|| async move {
            let Ok(res) = get("/api/me").send().await else {
                return None;
            };

            res.json::<User>().await.ok()
        });
        Self(me)
    }
}
