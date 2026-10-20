use leptos::prelude::*;

#[component]
pub fn RegisterPage() -> impl IntoView {
    view! {
        <section aria-labelledby="register-title">
            <h1 id="register-title">"Create your account"</h1>
            <p>"Register with your NEU Outlook account. If you already have an account, Microsoft will sign you in."</p>
            <a class="button" href="/api/auth/login">"Continue with Microsoft"</a>
        </section>
    }
}
