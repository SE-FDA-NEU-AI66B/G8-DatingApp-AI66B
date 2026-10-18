#[cfg(target_arch = "wasm32")]
use super::api;
use crate::share::admin::ActivityLogEntry;
use leptos::prelude::*;

#[component]
pub fn AdminActivityLogPage() -> impl IntoView {
    let (entries, set_entries) = signal(Vec::<ActivityLogEntry>::new());
    let (error, set_error) = signal(None::<String>);
    #[cfg(not(target_arch = "wasm32"))]
    let _ = (&set_entries, &set_error);

    #[cfg(target_arch = "wasm32")]
    leptos::task::spawn_local(async move {
        match api::get_json::<Vec<ActivityLogEntry>>("/api/admin/activity-log").await {
            Ok(value) => set_entries.set(value),
            Err(message) => set_error.set(Some(message)),
        }
    });

    view! {
        <section aria-labelledby="admin-activity-title">
            <h1 id="admin-activity-title">"Activity log"</h1>
            {move || error.get().map(|message| view! { <p role="alert">{message}</p> })}
            <ul>
                {move || entries.get().into_iter().map(|entry| view! {
                    <li>
                        <time>{entry.created_at}</time>
                        " — "
                        {entry.action}
                        {entry.target_id.map(|id| format!(" (target {id})"))}
                    </li>
                }).collect_view()}
            </ul>
        </section>
    }
}
