use itertools::Itertools;
use leptos::reactive::spawn_local;
use leptos::server::codee::string::FromToStringCodec;
use leptos::{html, prelude::*};
use leptos_use::use_cookie;
use sqlx::types::uuid::Uuid;
pub fn get_client_info() -> String {
    use wasm_bindgen::prelude::*;
    let window = web_sys::window().unwrap();
    let navigator = window.navigator();
    let user_agent = navigator.user_agent();
    // let user_agent = 1;
    format!("{user_agent:?}{window:?}")
}
pub fn App() -> impl IntoView {
    view! { <Forms /> }
}
pub fn Applogout() -> impl IntoView {
    let login_cookie = use_cookie::<String, FromToStringCodec>("login_cookie");
    Effect::new(move |_| {
        login_cookie.1.set(None);
        // navigate("/login", Default::default());
    });
    view! {
        <p>"Logging out…"</p>
        {login_cookie.0}
    }
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
    let mut cookie: Vec<u8> = vec![0; 32];
    getrandom::fill(&mut cookie).unwrap();
    println!("{:?}", cookie);
    let username = "mq".to_string();
    let password = "urmom_fat".to_string();
    let userid = sqlx::query_as("SELECT id from app_user where username=$1 and password=$2")
        .bind(username)
        .bind(password)
        .fetch_one(db)
        .await;
    if userid.is_err() {
        panic!("what {:?}", userid.unwrap_err());
    }
    let userid: (Uuid,) = match userid {
        Ok(userid) => userid,
        e => return Err(ServerFnError::new("Wronguser name or pass")),
    };
    let result = match sqlx::query(
        "INSERT INTO cookie_login (cookie,userid,info) VALUES ($1,$2,$3)
",
    )
    .bind(cookie.clone())
    .bind(userid.0)
    .bind(info.as_bytes())
    .execute(db)
    .await
    {
        Ok(a) => a,
        a => {
            println!("{:?}", a);
            return Err(ServerFnError::new("server err"));
        }
    };

    print!("{:?}", userid);
    // user_id(&cookie);
    Ok(hex::encode(&cookie))
}
pub fn Forms() -> impl IntoView {
    let username = signal("".to_string());
    let password = signal("".to_string());
    let device_info = signal("".to_string());
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
        <Show
            when=move || login_cookie.0.get().is_none()
            fallback=move || {
                view! {
                    <p>"You are logged in."</p>
                    <button on:click=move |_| login_cookie.1.set(None)>"Log out"</button>
                }
            }
        >
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
        </Show>
    }
}
