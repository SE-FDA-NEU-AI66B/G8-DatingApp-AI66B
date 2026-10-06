#[allow(unused_imports)]
use itertools::Itertools;
use leptos::prelude::*;
use leptos::reactive::spawn_local;
use leptos::server::codee::string::FromToStringCodec;
// mod e1;
// mod e2;
// use leptos::server_fn::client;
// #[client]
pub fn get_client_info() -> String {
    use wasm_bindgen::prelude::*;
    let window = web_sys::window().unwrap();
    let navigator = window.navigator();
    let user_agent = navigator.user_agent();
    // let user_agent = 1;
    format!("{user_agent:?}{window:?}")
}
pub fn App() -> impl IntoView {
    // let text=;
    view! { <Forms /> }
}
#[server()]
pub async fn login(
    username: String,
    password: String,
    info: String,
) -> Result<String, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    let db: Data<crate::share::database::Database> = extract().await.unwrap_or_else(|i| {
        println!("{:?}", i);
        panic!("asdf");
    });
    let db = (*db).0.as_ref();
    use sqlx::Row;
    println!("whatappp");
    // SELECT * from app_user where username='{"mq"}' and password='{"urmom_fat"}';
    let userid: (String,) =
        sqlx::query_as("SELECT username from app_user where username=$1 and password=$2")
            .bind(username)
            .bind(password)
            .fetch_one(db)
            .await
            .unwrap();
    print!("{:?}", userid);
    Ok("a cookie".to_string())
}
pub fn Forms() -> impl IntoView {
    let username = signal("".to_string());
    let password = signal("".to_string());
    let device_info = signal("".to_string());

    use leptos_use::use_cookie;
    let login_cookie = use_cookie::<String, FromToStringCodec>("login_cookie");

    use leptos::tachys::html::event::SubmitEvent;
    let on_submit = move |ev: SubmitEvent| {
        let login_cookie = login_cookie.clone();
        let a = get_client_info();
        device_info.1.set(a.clone());
        spawn_local(async move {
            let cookie = login(username.0.get(), password.0.get(), a).await.unwrap();
            login_cookie.1.set(Some(cookie));
        });
        ev.prevent_default();
    };
    view! {
        {device_info.0}
        <form on:submit=on_submit>
            <input type="text" placeholder="username" bind:value=username />
            <br />
            <input type="text" placeholder="password" bind:value=password />
            <br />
            <input type="submit" value="Submit" />
            <br />
            login_cookie:
            {login_cookie.0}
        </form>
    }
}
