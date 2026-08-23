#[allow(unused_imports)]
use itertools::Itertools;
use leptos::prelude::*;
pub fn App() -> impl IntoView {
    #[derive(Debug, Clone)]
    struct DatabaseEntry {
        key: String,
        value: i32,
    }
    let (data, set_data) = signal(vec![
        DatabaseEntry {
            key: "foo".to_string(),
            value: 10,
        },
        DatabaseEntry {
            key: "bar".to_string(),
            value: 20,
        },
        DatabaseEntry {
            key: "baz".to_string(),
            value: 15,
        },
    ]);
    view! {
        <br />
        // when we click, update each row,
        // doubling its value
        <button on:click=move |_| {
            set_data
                .update(|data| {
                    for row in data {
                        row.value *= 2;
                    }
                });
            leptos::logging::log!("{:?}", data.get());
        }>"Update Values"</button>
        // iterate over the rows and display each value
        <For each=move || data.get() key=|state| (state.key.clone(), state.value) let(child)>
            <p>{child.key}:{child.value}</p>
        </For>
    }
}
