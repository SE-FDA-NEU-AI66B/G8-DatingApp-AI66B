use crate::share::auth::AuthError;
use leptos::prelude::*;
use leptos::reactive::spawn_local;
use leptos::tachys::html::event::SubmitEvent;
use serde::Serialize;

#[derive(Serialize)]
struct PasswordLogin {
    username_or_email: String,
    password: String,
}

async fn post_password_login(username_or_email: String, password: String) -> Result<(), String> {
    use gloo_net::http::Request;

    let request = Request::post("/api/auth/password")
        .json(&PasswordLogin {
            username_or_email,
            password,
        })
        .map_err(|_| "Could not prepare the sign-in request.".to_string())?;
    let response = request
        .send()
        .await
        .map_err(|_| "Unable to reach the sign-in service.".to_string())?;
    if response.ok() {
        Ok(())
    } else {
        Err(response
            .json::<AuthError>()
            .await
            .map(|error| error.message)
            .unwrap_or_else(|_| "Sign-in failed.".to_string()))
    }
}

async fn post_logout() -> Result<(), String> {
    use gloo_net::http::Request;

    let response = Request::post("/api/auth/logout")
        .send()
        .await
        .map_err(|_| "Unable to reach the logout service. Please try again.".to_string())?;
    if response.ok() {
        Ok(())
    } else {
        Err("Unable to end the session.".to_string())
    }
}

#[component]
pub fn App() -> impl IntoView {
    let auth_state = use_context::<RwSignal<Option<bool>>>().expect("auth state context");
    let (username_or_email, set_username_or_email) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (error, set_error) = signal(None::<String>);
    let (busy, set_busy) = signal(false);

    let on_submit = move |event: SubmitEvent| {
        event.prevent_default();
        set_error.set(None);
        let username_or_email = username_or_email.get_untracked().trim().to_lowercase();
        let password = password.get_untracked();
        set_busy.set(true);
        spawn_local(async move {
            match post_password_login(username_or_email, password).await {
                Ok(()) => {
                    auth_state.set(Some(true));
                }
                Err(message) => set_error.set(Some(message)),
            }
            set_busy.set(false);
        });
    };

    let on_logout = move |_| {
        set_busy.set(true);
        set_error.set(None);
        spawn_local(async move {
            match post_logout().await {
                Ok(()) => {
                    auth_state.set(Some(false));
                }
                Err(message) => set_error.set(Some(message)),
            }
            set_busy.set(false);
        });
    };

    view! {
        <section aria-labelledby="login-title">
            <h1 id="login-title">"Sign in"</h1>
            {move || {
                if auth_state.get() == Some(true) {
                    view! {
                        <p role="status">"You are signed in."</p>
                        <button type="button" on:click=on_logout disabled=move || busy.get()>
                            {move || if busy.get() { "Signing out…" } else { "Sign out" }}
                        </button>
                    }.into_any()
                } else {
                    view! {
                        <form novalidate on:submit=on_submit>
                            <label for="login-username">"Username or NEU email"</label>
                            <input id="login-username" type="text" autocomplete="username"
                                placeholder="username or you@neu.edu.vn" bind:value=(username_or_email, set_username_or_email) required />
                            <label for="login-password">"Password"</label>
                            <input id="login-password" type="password" autocomplete="current-password"
                                bind:value=(password, set_password) required />
                            <button type="submit" disabled=move || busy.get()>
                                {move || if busy.get() { "Signing in…" } else { "Sign in" }}
                            </button>
                        </form>
                        <p>"Or sign in and verify with NEU Outlook:"</p>
                        <a class="button" href="/api/auth/login">"Continue with Microsoft"</a>
                    }.into_any()
                }
            }}
            <p role="alert" aria-live="polite">{move || error.get()}</p>
            <p>"New here? "<a href="/register">"Create an account"</a></p>
        </section>
    }
}

#[component]
pub fn Applogout() -> impl IntoView {
    let auth_state = use_context::<RwSignal<Option<bool>>>().expect("auth state context");
    let (message, set_message) = signal("Signing out…".to_string());
    Effect::new(move |_| {
        spawn_local(async move {
            match post_logout().await {
                Ok(()) => {
                    auth_state.set(Some(false));
                    set_message.set("You are signed out.".to_string());
                }
                Err(error) => set_message.set(error),
            }
        });
    });
    view! { <p role="status">{move || message.get()}</p> }
}
