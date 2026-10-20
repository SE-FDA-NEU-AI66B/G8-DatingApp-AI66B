use leptos::prelude::*;
use leptos::reactive::spawn_local;

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
    let (signed_in, set_signed_in) = signal(false);
    let (busy, set_busy) = signal(false);

    let on_logout = move |_| {
        set_busy.set(true);
        spawn_local(async move {
            if post_logout().await.is_ok() {
                set_signed_in.set(false);
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
                    }.into_any()
                } else {
                    view! {
                        <p>"Use your NEU Outlook account to sign in or create an account."</p>
                        <a class="button" href="/api/auth/login">"Continue with Microsoft"</a>
                    }.into_any()
                }
            }}
            <p>"New here? "<a href="/register">"Register with Microsoft"</a></p>
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
