use leptos::prelude::*;

mod e1;
mod e2;
pub fn App() -> impl IntoView {
    view! {
        {e1::App()}
        <e2::App />
    }
}
