use leptos::{
    ev,
    html::{self, dialog, div},
    prelude::*,
};
use uuid::Uuid;

#[component]
pub fn Modal(
    children: Children,
    open: ReadSignal<bool>,
    set_open: WriteSignal<bool>,
) -> impl IntoView {
    let id = Uuid::new_v4();
    let dialog_ref: NodeRef<html::Dialog> = NodeRef::new();

    let _foo = dialog()
        .node_ref(dialog_ref)
        .on(ev::close, move |_| {
            set_open(false);
        })
        .class("modal")
        .id(id.to_string())
        .child(div().class("modal-box").child(children()));

    let _ = Effect::watch(
        move || open.get(),
        move |modal_open, _, _| {
            if *modal_open {
                let _ = dialog_ref.get_untracked().unwrap().show_modal();
            } else {
                dialog_ref.get_untracked().unwrap().close();
            }
        },
        false,
    );

    _foo
}
