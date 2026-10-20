use crate::share::register::{normalize_email, MessageBody, RegisterReq};
use leptos::prelude::*;
use leptos::reactive::spawn_local;
use leptos::tachys::html::event::SubmitEvent;

async fn post_register(body: RegisterReq) -> Result<String, String> {
    use gloo_net::http::Request;

    let request = Request::post("/api/register")
        .json(&body)
        .map_err(|_| "Could not prepare the registration request.".to_string())?;
    let response = request
        .send()
        .await
        .map_err(|_| "Unable to reach the registration service.".to_string())?;
    let message = response
        .json::<MessageBody>()
        .await
        .map(|body| body.message)
        .unwrap_or_else(|_| "Unexpected server response.".to_string());
    if (200..300).contains(&response.status()) {
        Ok(message)
    } else {
        Err(message)
    }
}

#[component]
pub fn RegisterPage() -> impl IntoView {
    let (display_name, set_display_name) = signal(String::new());
    let (email, set_email) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (error, set_error) = signal(None::<String>);
    let (success, set_success) = signal(None::<String>);
    let (busy, set_busy) = signal(false);

    let on_submit = move |event: SubmitEvent| {
        event.prevent_default();
        set_error.set(None);
        set_success.set(None);
        let email = match normalize_email(&email.get_untracked()) {
            Ok(email) => email,
            Err(message) => {
                set_error.set(Some(message.into()));
                return;
            }
        };
        let body = RegisterReq {
            email,
            display_name: display_name.get_untracked(),
            password: password.get_untracked(),
        };
        set_busy.set(true);
        spawn_local(async move {
            match post_register(body).await {
                Ok(message) => set_success.set(Some(message)),
                Err(message) => set_error.set(Some(message)),
            }
            set_busy.set(false);
        });
    };

    view! {
        <section aria-labelledby="register-title">
            <h1 id="register-title">"Create your account"</h1>
            <form novalidate on:submit=on_submit>
                <label for="register-name">"Name"</label>
                <input id="register-name" type="text" autocomplete="name" maxlength="80"
                    bind:value=(display_name, set_display_name) required />
                <label for="register-email">"NEU Outlook email"</label>
                <input id="register-email" type="email" autocomplete="email"
                    placeholder="you@neu.edu.vn" bind:value=(email, set_email) required />
                <label for="register-password">"Password"</label>
                <input id="register-password" type="password" autocomplete="new-password"
                    minlength="12" maxlength="1024" bind:value=(password, set_password) required />
                <button type="submit" disabled=move || busy.get()>
                    {move || if busy.get() { "Creating account…" } else { "Create account" }}
                </button>
            </form>
            <p role="alert" aria-live="polite">{move || error.get()}</p>
            <p role="status" aria-live="polite">{move || success.get()}</p>
            {move || success.get().map(|_| view! {
                <a class="button" href="/api/auth/login">"Verify with NEU Outlook"</a>
            })}
            <p>"Already registered? "<a href="/login">"Sign in"</a></p>
        </section>
    }
}
