#[allow(unused_imports)]
use itertools::Itertools;
use leptos::prelude::*;
pub fn iteration() -> impl IntoView {
    let values = vec![0, 1, 2];
    // let (count,set_count)=signal(0);
    let counters = (0..10).map(|idx| RwSignal::new(idx)).collect_vec();
    let counter_buttons = counters
        .iter()
        .map(|&count| {
            view! {
                <li>
                    <button on:click=move |_| *count.write() += 1>{count}</button>
                </li>
            }
        })
        .collect_vec();

    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    struct Counter {
        id: usize,
        count: RwSignal<i32>,
    }
    let counters2 = (0..4)
        .map(|idx| Counter {
            id: idx,
            count: RwSignal::new(0),
        })
        .collect_vec();
    let f = move |index: ReadSignal<usize>, counter: Counter| {
        view! {
            <button on:click=move |_| {
                *counter.count.write() += 1;
            }>{move || index.get()} ". Value: " {move || counter.count.get()}</button>
        }
    };

    // let a = counters2.iter().map(|i| &i);
    let counters3 = counters2.clone();
    view! {
        {counter_buttons}
        <p>{values.clone()}</p>
        <ul>{values.into_iter().map(|n| view! { <li>{n}</li> }).collect_vec()}</ul>
        // Same as <For/>
        <ForEnumerate each=move || counters2.clone() key=move |counter| counter.id children=f />
        <br />
        <ForEnumerate each=move || counters3.clone() key=move |counter| counter.id children=f />
    }
}
//
// Provides the index as a signal and the child T
// children=move |index: ReadSignal<usize>, counter: Counter| {
//     view! {
//         <button>{move || index.get()} ". Value: " {move || counter.count.get()}</button>
//     }
// }
