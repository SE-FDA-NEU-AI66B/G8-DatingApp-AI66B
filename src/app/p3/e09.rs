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
        <Buttona setter=set_toggled />
        <Buttonb on_click=move |_| set_toggled.update(|value| *value = !*value) />
        <Buttonc on:click=move |_| set_toggled.update(|value| *value = !*value) />
        <Layout set_toggled />
        <Buttond />
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
            <Buttona setter=set_toggled />
        </div>
    }
}

use leptos::tachys::html::event;

#[component]
pub fn buttona(setter: WriteSignal<bool>) -> impl IntoView {
    view! { <button on:click=move |_| *setter.write() ^= true>"Toggle"</button> }
}

#[component]
pub fn buttonb(on_click: impl FnMut(event::MouseEvent) + 'static) -> impl IntoView {
    view! { <button on:click=on_click>"Toggle"</button> }
}
#[component]
pub fn buttonc() -> impl IntoView {
    view! { <button>"Toggle"</button> }
}
#[component]
pub fn buttond() -> impl IntoView {
    let setter = use_context::<WriteSignal<bool>>().expect("to have found the setter provided");
    view! { <button on:click=move |_| *setter.write() ^= true>"Toggle"</button> }
}
