use leptos::server::Resource;

use crate::request::get;

#[derive(Clone)]
pub struct AuthContext(pub Resource<bool>);

// TODO(seb): Fix this
// get("/api/me").send().await.is_ok_and(|r| r.ok())
impl AuthContext {
    pub fn setup() -> Self {
        let foo = Resource::new(|| (), |_| async move { true });
        Self(foo)
    }
}
