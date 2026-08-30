use leptos::prelude::*;
use leptos::reactive::spawn_local;
use serde::Deserialize;
#[derive(Deserialize, Debug)]
struct MyQuery {
    name: String,
}
// use server_fn::server_fn_error
#[server]
pub async fn actix_extract() -> Result<String, ServerFnError> {
    use actix_web::dev::ConnectionInfo;
    use actix_web::http::Method;
    use actix_web::web::Query;
    use leptos_actix::extract;
    let (connection): (ConnectionInfo) = extract().await?;
    let worker: Query<crate::worker::Worker> = extract().await?;
    let r = format!("connection={connection:?}");
    // let r = String::from("Asdffasdfasdf");
    println!("{:}", r);
    println!("fasdfasdf");
    Ok(r)
}
// leptos::server::codee::string::codee
pub fn App() -> impl IntoView {
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
