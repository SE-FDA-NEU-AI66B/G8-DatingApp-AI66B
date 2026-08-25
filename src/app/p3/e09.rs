// #![allow(non_snake_case)]
#[allow(unused_imports)]
use itertools::Itertools;
use leptos::prelude::*;
#[component]
pub fn App() -> impl IntoView {
    let (toggled, set_toggled) = signal(false);

    provide_context(set_toggled);
    view! {
        <p>"Toggled? " {toggled}</p>
        <ButtonA setter=set_toggled />
        <ButtonB on_click=move |_| set_toggled.update(|value| *value = !*value) />
        <ButtonC on:click=move |_| set_toggled.update(|value| *value = !*value) />
        <Layout set_toggled />
        <ButtonD />
    }
}
#[component]

pub fn Layout(set_toggled: WriteSignal<bool>) -> impl IntoView {
    view! {
        <header>
            <h1>"My Page"</h1>
        </header>
        <main>
            <Content set_toggled />
        </main>
    }
}
#[component]
pub fn Content(set_toggled: WriteSignal<bool>) -> impl IntoView {
    view! {
        <div class="content">
            <ButtonA setter=set_toggled />
        </div>
    }
}

use leptos::tachys::html::event;

#[component]
pub fn ButtonA(setter: WriteSignal<bool>) -> impl IntoView {
    view! { <button on:click=move |_| *setter.write() ^= true>"Toggle"</button> }
}

#[component]
pub fn ButtonB(on_click: impl FnMut(event::MouseEvent) + 'static) -> impl IntoView {
    view! { <button on:click=on_click>"Toggle"</button> }
}
#[component]
pub fn ButtonC() -> impl IntoView {
    view! { <button>"Toggle"</button> }
}
pub fn ButtonD() -> impl IntoView {
    let setter = use_context::<WriteSignal<bool>>().expect("to have found the setter provided");
    view! { <button on:click=move |_| *setter.write() ^= true>"Toggle"</button> }
}
