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
    use actix_web::dev::ConnectionInfo;
    use actix_web::http::header;
    use actix_web::http::Method;
    use actix_web::web::Header;
    use actix_web::web::{Data, Query};
    use leptos_actix::extract;

    let (connection): (ConnectionInfo) = extract().await.unwrap_or_else(|i| {
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
pub fn App() -> impl IntoView {
    use leptos::server::codee::string::FromToStringCodec;
    use leptos_use::use_cookie;
    let (counter, set_counter) = use_cookie::<u32, FromToStringCodec>("counter");
    let reset = move || set_counter.set(Some(34234));

    if counter.get().is_none() {
        reset();
    }

    let (query, query_set) = signal(String::from("Asdf"));
    let f = move |_| {
        spawn_local(async move {
            let a = actix_extract().await;
            query_set(a.ok().unwrap());
        });
    };
    view! {
        <br />
        <button on:click=f>{query}</button>
        <br />
    }
}
