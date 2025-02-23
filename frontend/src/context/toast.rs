use crate::components::icons::{
    error_icon::ErrorIcon, success_icon::SuccessIcon, warning_icon::WarningIcon,
};
use anyhow::anyhow;
use leptos::prelude::ElementChild;
use leptos::prelude::*;
use leptos::prelude::{provide_context, use_context, Update, WriteSignal};
use std::time::Duration;

#[derive(Clone)]
pub enum ToastType {
    Success,
    Warning,
    Error,
}

#[component]
pub fn Toaster(children: Children) -> impl IntoView {
    let (toaster, set_toaster) = signal(Toaster::new());
    let (removed, set_removed) = signal(vec![]);
    provide_context(set_toaster);

    let t = move || {
        toaster()
            .alerts
            .iter()
            .map(|a| {
                let toast = &a.0;
                if let Some(timeout) = toast.timeout {
                    let id = a.1;
                    if !removed().contains(&id) {
                        set_timeout(move || set_toaster.update(|t| t.remove(id)), timeout);
                        set_removed.update(|v| v.push(id));
                    }
                }
                toast.get()
            })
            .collect::<Vec<_>>()
    };

    view! {
        <>
            <div class="toast toast-bottom toast-center sm:min-w-[640px] min-w-full z-[999999]">
                {t}
            </div>
            {children()}
        </>
    }
}

#[derive(Clone)]
pub struct Toast {
    pub ty: ToastType,
    pub body: String,
    pub timeout: Option<Duration>,
}

impl Toast {
    pub fn get(&self) -> impl IntoView {
        let (icon, alert_type) = match self.ty {
            ToastType::Success => (SuccessIcon.into_any(), "alert-success"),
            ToastType::Warning => (WarningIcon.into_any(), "alert-warning"),
            ToastType::Error => (ErrorIcon.into_any(), "alert-error"),
        };

        view! {
            <div
                role="alert"
                class=format!(
                    "grid-flow-col grid-cols-[auto_minmax(auto,1fr)] justify-items-start text-start text-wrap alert {}",
                    alert_type,
                )
            >

                {icon}
                <span>{self.body.to_string()}</span>
            </div>
        }
    }
}

#[derive(Clone)]
pub struct Toaster {
    pub alerts: Vec<(Toast, i32)>,
    last_id: i32,
}

impl Toaster {
    pub fn new() -> Self {
        Toaster {
            alerts: vec![],
            last_id: 0,
        }
    }

    fn remove(&mut self, id: i32) {
        self.alerts.retain(|it| it.1 != id);
    }

    pub fn add(&mut self, toast: Toast) {
        self.last_id += 1;
        self.alerts.push((toast, self.last_id));
    }
}

pub trait ToasterTrait {
    fn add(&self, toast: Toast);
}

impl ToasterTrait for WriteSignal<Toaster> {
    fn add(&self, toast: Toast) {
        self.update(|t| t.add(toast))
    }
}

pub fn use_toast() -> Result<WriteSignal<Toaster>, anyhow::Error> {
    use_context::<WriteSignal<Toaster>>().ok_or_else(|| anyhow!("Couldn't find context"))
}
