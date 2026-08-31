use leptos::prelude::*;
use leptos::reactive::spawn_local;
use serde::Deserialize;
#[derive(Deserialize, Debug)]
struct MyQuery {
    name: String,
}
// use server_fn::server_fn_error
#[server(endpoint = "urmom_is_fat")]
pub async fn actix_extract() -> Result<String, ServerFnError> {
    println!("sadfasdf");
    use actix_web::dev::ConnectionInfo;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let (connection): ConnectionInfo = extract().await.unwrap_or_else(|i| {
        println!("{:?}", i);
        panic!("asdf");
    });
    let worker: (Data<crate::share::worker::Worker>, Data<LeptosOptions>) =
        extract().await.unwrap_or_else(|i| {
            println!("{:?}", i);
            panic!("asdf");
        });
    let r = format!("connection={connection:?},worker={worker:?}");
    Ok(r)
}
// leptos::server::codee::string::codee
#[component]
pub fn App() -> impl IntoView {
    use codee::string::FromToStringCodec;
    use leptos_use::{use_cookie, use_cookie_with_options, SameSite, UseCookieOptions};
    let (counter, set_counter) = use_cookie::<u32, FromToStringCodec>("counter");
    let reset = move || set_counter.set(Some(34234));
    if counter.get().is_none() {
        reset();
    }
    let increase = move || {
        set_counter.set(counter.get().map(|c| c + 1));
    };
    let (query, query_set) = signal(String::from("Asdf"));
    let f = move |_| {
        spawn_local(async move {
            let a = actix_extract().await;
            query_set(a.ok().unwrap());
        });
    };
    leptos::prelude::view! {
        <br />
        <br />
        <button on:click=f>{query}</button>
        <br />
    <p>Counter: {move || counter.get().map(|c| c.to_string()).unwrap_or("—".to_string())}</p>
    <button on:click=move |_| reset()>Reset</button>
    <button on:click=move |_| increase()>+</button>
    }
}
