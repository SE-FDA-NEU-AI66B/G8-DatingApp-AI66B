use crate::share::auth::{AuthError, LoginRequest};
use crate::share::register::normalize_email;
use leptos::prelude::*;
use leptos::reactive::spawn_local;
use leptos::tachys::html::event::SubmitEvent;

async fn post_login(email: String, otp: String) -> Result<(), String> {
    use gloo_net::http::Request;

    let request = Request::post("/api/auth/login")
        .json(&LoginRequest { email, otp })
        .map_err(|_| "Could not prepare the login request.".to_string())?;
    let response = request
        .send()
        .await
        .map_err(|_| "Unable to reach the login service. Please try again.".to_string())?;

    if response.ok() {
        Ok(())
    } else {
        let error = response.json::<AuthError>().await.ok();
        Err(error.map_or_else(
            || "Login failed. Please check your email and OTP.".to_string(),
            |error| error.message,
        ))
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
    let (email, set_email) = signal(String::new());
    let (otp, set_otp) = signal(String::new());
    let (error, set_error) = signal(None::<String>);
    let (signed_in, set_signed_in) = signal(false);
    let (busy, set_busy) = signal(false);

    let on_submit = move |event: SubmitEvent| {
        event.prevent_default();
        set_error.set(None);

        let normalized_email = match normalize_email(&email.get_untracked()) {
            Ok(email) => email,
            Err(message) => {
                set_error.set(Some(message.to_string()));
                return;
            }
        };
        let otp_value = otp.get_untracked().trim().to_string();
        if otp_value.len() != 6 || !otp_value.chars().all(|character| character.is_ascii_digit()) {
            set_error.set(Some("Enter the six-digit OTP sent to your email.".to_string()));
            return;
        }

        set_busy.set(true);
        spawn_local(async move {
            match post_login(normalized_email, otp_value).await {
                Ok(()) => set_signed_in.set(true),
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
                Ok(()) => set_signed_in.set(false),
                Err(message) => set_error.set(Some(message)),
            }
            set_busy.set(false);
        });
    };

    view! {
        <section aria-labelledby="login-title">
            <h1 id="login-title">"Sign in"</h1>
            {move || {
                if signed_in.get() {
                    view! {
                        <p role="status">"You are signed in."</p>
                        <button type="button" on:click=on_logout disabled=move || busy.get()>
                            {move || if busy.get() { "Signing out…" } else { "Sign out" }}
                        </button>
                    }
                        .into_any()
                } else {
                    view! {
                        <form novalidate on:submit=on_submit>
                            <label for="login-email">"NEU email"</label>
                            <input
                                id="login-email"
                                type="email"
                                autocomplete="email"
                                placeholder="you@neu.edu.vn"
                                bind:value=(email, set_email)
                                required
                            />
                            <label for="login-otp">"Six-digit OTP"</label>
                            <input
                                id="login-otp"
                                type="text"
                                inputmode="numeric"
                                autocomplete="one-time-code"
                                maxlength="6"
                                placeholder="123456"
                                bind:value=(otp, set_otp)
                                required
                            />
                            <button type="submit" disabled=move || busy.get()>
                                {move || if busy.get() { "Signing in…" } else { "Sign in" }}
                            </button>
                        </form>
                    }
                        .into_any()
                }
            }}
            <p role="alert" aria-live="polite">{move || error.get()}</p>
            <p>"New here? "<a href="/register">"Create an account"</a></p>
        </section>
    }
}

#[component]
pub fn Applogout() -> impl IntoView {
    let (message, set_message) = signal("Signing out…".to_string());
    Effect::new(move |_| {
        spawn_local(async move {
            set_message.set(match post_logout().await {
                Ok(()) => "You are signed out.".to_string(),
                Err(error) => error,
            });
        });
    });
    view! { <p role="status">{move || message.get()}</p> }
}
