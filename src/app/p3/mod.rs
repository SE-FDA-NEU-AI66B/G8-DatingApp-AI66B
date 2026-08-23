#[allow(unused_imports)]
use itertools::Itertools;
use leptos::prelude::*;
mod e4;
mod e5;
mod e6;
mod e7;
mod e9;
#[component]
pub fn ProgressBar(
    ///The maximum value of the progress bar.
    #[prop(default = 400)]
    max: u16,
    #[prop(into)] progress: Signal<i32>,
    // impl Fn() -> i32 + Send + Sync + 'static,
) -> impl IntoView {
    view! { <progress max=max value=progress /> }
}
// fn spread_onto_component() -> impl Attribute {
//
// }
#[component]
pub fn App() -> impl IntoView {
    let html = "<p> This HTML will be injected.</p>";
    let (count, set_count) = signal(0);
    let double_count = Signal::derive(move || count.get() * 2);
    view! {
        <button on:click=move |_| *set_count.write() += 1>"click me: " {count}</button>
        <p class:red=(move || count.get() & 1 == 1)>"Double count: " {move || count.get() * 2}</p>
        <p class=("red", move || count.get() & 1 == 1)>"Double count: " {double_count}</p>
        <button
            on:click=move |_| *set_count.write() += 10
            style="position: absolute"
            style:left=move || format!("{}px", count.get() + 300)
            style:max-witdh="400px"
            style:background-color=move || format!("rgb({},{},100)", count.get(), 100)
            style=("--columns", move || count.get().to_string())
        >
            "Click to Move"
        </button>
        <br />
        <progress max=50 value=double_count></progress>
        <br />
        <progress max=50 value=count></progress>
        <br />
        <ProgressBar progress=count></ProgressBar>
        <ProgressBar progress=double_count></ProgressBar>
        <div inner_html=html />
        {e4::iteration()}
        {e5::App()}
        {e6::forms()}
        {e5::App()}
        <e7::App />
        <br />
        <e9::App />
        <e9::App />
        <e9::App />
    }
}
