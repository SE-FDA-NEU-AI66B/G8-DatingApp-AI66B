use leptos::prelude::*;
use leptos::reactive::spawn_local;
// use server_fn::server_fn_error
#[server]
pub async fn add_todo(title: String) -> Result<(), ServerFnError> {
    println!("fasdfasdf");
    // let mut conn=db()
    // {
    // }
    Ok(())
}
// leptos::server::codee::string::codee
pub fn App() -> impl IntoView {
    view! {
        <br />
        <button on:click=|_| {
            spawn_local(async {
                add_todo("asdf".to_string()).await;
            });
        }>ssf</button>
        <br />
    }
}
