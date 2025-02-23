use leptos::prelude::*;
use std::rc::Rc;

#[derive(Clone)]
pub struct StepStruct {
    label: String,
    child: Rc<dyn Fn() -> AnyView>,
}

#[component]
pub fn Stepper() -> impl IntoView {
    view! {}
    // let (step, set_step) = signal(starting_step);
    //
    // let children = children()
    //     .as_children()
    //     .iter()
    //     .map(|child| {
    //         child
    //             .as_transparent()
    //             .and_then(|t| t.downcast_ref::<StepStruct>())
    //             .expect("Child of `<Stepper />` should only be `<Step />`")
    //     })
    //     .cloned()
    //     .collect::<Vec<_>>();
    //
    // let internal_children = children.clone();
    // let children_len = children.len();
    //
    // let current_step = move || internal_children[step()].child.clone();
    //
    // view! {
    //     <ul class="steps">
    //         {children
    //             .into_iter()
    //             .enumerate()
    //             .map(|(i, s)| {
    //                 let class = move || if i <= step() { "step step-primary" } else { "step" };
    //                 view! {
    //                     <li
    //                         on:click=move |_| {
    //                             if step() != i {
    //                                 set_step(i);
    //                             }
    //                         }
    //
    //                         class=class
    //                     >
    //                         {s.label}
    //                     </li>
    //                 }
    //             })
    //             .collect::<Vec<_>>()}
    //
    //     </ul>
    //
    //     {current_step}
    //
    //     <div class="btm-nav bg-neutral">
    //         <button
    //             type="button"
    //             on:click=move |_| {
    //                 if step() > 0 {
    //                     set_step(step() - 1);
    //                 }
    //             }
    //         >
    //
    //             {move || { if step() > 0 { "Previous".into_any() } else { ().into_any() } }}
    //         </button>
    //         <button
    //             type="button"
    //             on:click=move |_| {
    //                 if step() < children_len - 1 {
    //                     set_step(step() + 1);
    //                 }
    //             }
    //         >
    //
    //             {move || {
    //                 if step() < children_len - 1 { "Next".into_any() } else { ().into_any() }
    //             }}
    //
    //         </button>
    //     </div>
    // }
}

// #[component(transparent)]
// pub fn Step<F, E>(label: &'static str, child: F) -> impl IntoView
// where
//     F: Fn() -> E + 'static,
//     E: IntoView,
// {
//     StepStruct {
//         label: label.to_string(),
//         child: Rc::new(move || child().into_any()),
//     }
// }
