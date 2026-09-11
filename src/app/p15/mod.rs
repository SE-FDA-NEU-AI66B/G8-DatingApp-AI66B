use leptos::prelude::*;

mod e1;
mod e2;
#[component]
pub fn app() -> impl IntoView {
    view! {
        {e1::App()}
        <e2::App />
    }
}
