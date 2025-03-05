use leptos::prelude::{AsyncDerived, LocalStorage};

use crate::request::get;

#[derive(Clone)]
pub struct AuthContext(pub AsyncDerived<bool, LocalStorage>);

impl AuthContext {
    pub fn setup() -> Self {
        let me = AsyncDerived::new_unsync(|| async move {
            get("/api/me").send().await.is_ok_and(|r| r.ok())
        });
        Self(me)
    }
}
