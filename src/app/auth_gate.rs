use leptos::prelude::*;
use leptos::reactive::spawn_local;
use leptos_router::hooks::use_navigate;

#[component]
pub fn AuthGate(children: ChildrenFn, #[prop(optional)] admin_only: bool) -> impl IntoView {
    let (authorized, set_authorized) = signal(false);
    let (load_failed, set_load_failed) = signal(false);
    let navigate = use_navigate();

    Effect::new(move |_| {
        let navigate = navigate.clone();
        spawn_local(async move {
            use gloo_net::http::Request;

            let endpoint = if admin_only {
                "/api/admin/health"
            } else {
                "/api/auth/session"
            };
            let status = match Request::get(endpoint).send().await {
                Ok(response) => response.status(),
                Err(_) => {
                    set_load_failed.set(true);
                    return;
                }
            };

            match status {
                200 => set_authorized.set(true),
                401 => navigate("/login", Default::default()),
                403 if admin_only => navigate("/", Default::default()),
                403 => navigate("/login", Default::default()),
                _ => set_load_failed.set(true),
            }
        });
    });

    view! {
        <Show
            when=move || authorized.get()
            fallback=move || view! {
                <p role="status">
                    {move || if load_failed.get() {
                        "Unable to check your sign-in. Please refresh and try again."
                    } else {
                        "Checking sign-in…"
                    }}
                </p>
            }
        >
            {children()}
        </Show>
    }
}
