use leptos::server::Resource;

use crate::request::get;

#[derive(Clone)]
pub struct AuthContext(pub Resource<bool>);

impl AuthContext {
    pub fn setup() -> Self {
        let foo = Resource::new(
            || (),
            |_| async move { get("/api/me").send().await.is_ok_and(|r| r.ok()) },
        );
        Self(foo)
    }
}
