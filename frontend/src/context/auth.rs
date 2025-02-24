use leptos::{
    prelude::{AsyncDerived, LocalStorage},
    server::Resource,
};

use crate::request::get;

#[derive(Clone)]
pub struct AuthContext(pub AsyncDerived<bool, LocalStorage>);

// TODO(seb): Fix this
// get("/api/me").send().await.is_ok_and(|r| r.ok())
impl AuthContext {
    pub fn setup() -> Self {
        let foo = AsyncDerived::new_unsync(|| async move {
            get("/api/me").send().await.is_ok_and(|r| r.ok())
        });
        Self(foo)
    }
}
