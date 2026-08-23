#[allow(unused_imports)]
use itertools::Itertools;
use leptos::prelude::*;
pub fn App() -> impl IntoView {
    use leptos::logging;
    let (value, set_value) = signal(0);
    let message = move || {
        if value.get() > 5 {
            // remember to check log in browser
            // logging::log!("{}: rendering Big Bad", value.get());
            "Big"
        } else {
            // logging::log!("{}: rendering Small Bad", value.get());
            "Small"
        }
    };
    let small = || {
        // logging::log!("rendering Small Good");
        "small"
    };

    view! {
        <button on:click=move |cx| *set_value.write() += 1>{message}:{value}</button>
        <button on:click=move |cx| *set_value.write() += 1>
            <Show when=move || { value.get() > 5 } fallback=small>
                big
            </Show>
        </button>
    }
}
