#[allow(unused_imports)]
use itertools::Itertools;
use leptos::prelude::*;
pub fn App() -> impl IntoView {
    use leptos::logging;
    let (a, set_a) = signal(0);
    let (b, set_b) = signal(0);

    Effect::new(move |_| {
        // immediately prints "Value: 0" and subscribes to `a`
        logging::log!("Value: {}", a.get());
    });
    view! { <button on:click=move |_| *set_a.write() += 1>{a}</button> }
}
