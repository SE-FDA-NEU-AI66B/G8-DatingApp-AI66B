use crate::share::register::{normalize_email, MessageBody, RegisterReq};
use leptos::prelude::*;
use leptos::reactive::spawn_local;
use leptos::tachys::html::event::SubmitEvent;

/// Chỉ chạy trong trình duyệt (gọi từ event handler).
async fn post_register(email: String) -> Result<String, String> {
    use gloo_net::http::Request;
    let req = Request::post("/api/register")
        .json(&RegisterReq { email })
        .map_err(|e| e.to_string())?;
    let resp = req
        .send()
        .await
        .map_err(|_| "Network error, please try again".to_string())?;
    let status = resp.status();
    let body: MessageBody = resp
        .json()
        .await
        .map_err(|_| "Unexpected server response".to_string())?;
    if status == 201 {
        Ok(body.message)
    } else {
        Err(body.message) // 400 / 409 / 500: hiển thị đúng message server trả
    }
}

#[component]
pub fn RegisterPage() -> impl IntoView {
    let (email, set_email) = signal(String::new());
    let (error, set_error) = signal(None::<String>);
    let (success, set_success) = signal(None::<String>);
    let (busy, set_busy) = signal(false);

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        set_error.set(None);
        set_success.set(None);

        // validate client: cùng hàm với server
        let email_val = match normalize_email(&email.get_untracked()) {
            Ok(e) => e,
            Err(m) => {
                set_error.set(Some(m.to_string()));
                return;
            }
        };

        set_busy.set(true);
        spawn_local(async move {
            match post_register(email_val).await {
                Ok(m) => set_success.set(Some(m)),
                Err(m) => set_error.set(Some(m)),
            }
            set_busy.set(false);
        });
    };

    view! {
        <h1>"Register"</h1>
        <form novalidate on:submit=on_submit>
            <input type="email" placeholder="you@neu.edu.vn" bind:value=(email, set_email) />
            <button type="submit" disabled=move || busy.get()>
                "Register"
            </button>
        </form>
        <p role="alert" style="color: crimson">
            {move || error.get()}
        </p>
        <p role="status" style="color: green">
            {move || success.get()}
        </p>
    }
}