#[allow(unused_imports)]
use itertools::Itertools;
use leptos::prelude::*;
#[allow(non_snake_case)]
pub fn App() -> impl IntoView {
    #[allow(unused_imports)]
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
        <button on:click=move |_| *set_value.write() += 1>{message}:{value}</button>
        <button on:click=move |_| *set_value.write() += 9>
            <Show when=move || { value.get() > 5 } fallback=small>
                big
            </Show>
        </button>
    }
}
