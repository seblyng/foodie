use leptos::prelude::*;

#[component]
pub fn Stepper(children: Steps) -> impl IntoView {
    let (step, set_step) = signal(0);

    let steps = children.0();
    let children_len = steps.len();
    let labels = steps.iter().map(|it| it.label.clone()).collect::<Vec<_>>();
    let _steps = steps.clone();

    let _step = move || _steps[step()].child.run();

    view! {
        <ul class="steps">
            {labels
                .into_iter()
                .enumerate()
                .map(|(i, label)| {
                    let class = move || if i <= step() { "step step-primary" } else { "step" };
                    view! {
                        <li
                            on:click=move |_| {
                                if step() != i {
                                    set_step(i);
                                }
                            }

                            class=class
                        >
                            {label}
                        </li>
                    }
                })
                .collect::<Vec<_>>()}

        </ul>

        {_step()}

        <div class="btm-nav bg-neutral">
            <button
                type="button"
                on:click=move |_| {
                    if step() > 0 {
                        set_step(step() - 1);
                    }
                }
            >

                {move || { if step() > 0 { "Previous".into_any() } else { ().into_any() } }}
            </button>
            <button
                type="button"
                on:click=move |_| {
                    if step() < children_len - 1 {
                        set_step(step() + 1);
                    }
                }
            >

                {move || {
                    if step() < children_len - 1 { "Next".into_any() } else { ().into_any() }
                }}

            </button>
        </div>
    }
}

#[derive(Clone)]
pub struct StepStruct {
    pub label: String,
    pub child: ViewFn,
}

#[component(transparent)]
pub fn Step(label: &'static str, #[prop(into)] child: ViewFn) -> StepStruct {
    StepStruct {
        label: label.to_string(),
        child,
    }
}

pub struct Steps(Box<dyn FnOnce() -> Vec<StepStruct>>);

impl<F, C> ToChildren<F> for Steps
where
    F: FnOnce() -> C + Send + 'static,
    C: IntoSteps,
{
    #[inline]
    fn to_children(f: F) -> Self {
        Steps(Box::new(move || f().into_steps()))
    }
}

// TODO(seb): Implement a macro which can implement this for different tuples
trait IntoSteps {
    fn into_steps(self) -> Vec<StepStruct>;
}

impl IntoSteps for (StepStruct, StepStruct, StepStruct) {
    fn into_steps(self) -> Vec<StepStruct> {
        vec![self.0, self.1, self.2]
    }
}
