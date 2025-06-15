use anyhow::anyhow;
use leptos::prelude::*;
use leptos::prelude::{provide_context, use_context};
use std::time::Duration;
use thaw::*;

#[derive(Clone)]
pub enum ToastType {
    Success,
    Warning,
    Error,
}

#[component]
pub fn Toaster(children: Children) -> impl IntoView {
    view! {
        <ToasterProvider>
            {
                let _toaster = ToasterInjection::expect_context();
                let toaster = RwSignal::new(Toaster::new(_toaster));
                provide_context(toaster);
                children()
            }
        </ToasterProvider>
    }
}

#[derive(Clone)]
pub struct Toast {
    pub ty: ToastType,
    pub body: String,
    pub timeout: Option<Duration>,
}

#[derive(Clone)]
pub struct Toaster {
    inj: ToasterInjection,
}

impl Toaster {
    pub fn new(toaster: ToasterInjection) -> Self {
        Toaster { inj: toaster }
    }
}

pub trait ToasterTrait {
    fn add(&self, toast: Toast);
}

impl ToasterTrait for RwSignal<Toaster> {
    fn add(&self, toast: Toast) {
        let intent = match toast.ty {
            ToastType::Success => ToastIntent::Success,
            ToastType::Warning => ToastIntent::Warning,
            ToastType::Error => ToastIntent::Error,
        };

        let mut opts = ToastOptions::default().with_intent(intent);
        if let Some(duration) = toast.timeout {
            opts = opts.with_timeout(duration);
        }

        self.get_untracked().inj.dispatch_toast(
            move || {
                view! {
                    <Toast>
                        <ToastTitle>{toast.body}</ToastTitle>
                    </Toast>
                }
            },
            opts,
        );
    }
}

pub fn use_toast() -> Result<RwSignal<Toaster>, anyhow::Error> {
    use_context::<RwSignal<Toaster>>().ok_or_else(|| anyhow!("Couldn't find context"))
}
