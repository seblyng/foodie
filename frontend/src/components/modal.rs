use leptos::{ev, prelude::*};
use uuid::Uuid;

#[component]
pub fn Modal(
    children: Children,
    open: ReadSignal<bool>,
    set_open: WriteSignal<bool>,
) -> impl IntoView {
    let id = Uuid::new_v4();

    let mut dialog = view! {
        <dialog id=id.to_string() class="modal">
            <div class="modal-box">{children()}</div>
        </dialog>
    };

    dialog = dialog.on(ev::close, move |_| {
        set_open(false);
    });

    let _dialog = dialog.clone();

    let _ = Effect::watch(
        move || open.get(),
        move |modal_open, _, _| {
            if *modal_open {
                let _ = _dialog.show_modal();
            } else {
                _dialog.close();
            }
        },
        false,
    );

    dialog
}
