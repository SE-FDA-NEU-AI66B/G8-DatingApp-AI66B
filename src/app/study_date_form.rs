use super::api;
use crate::share::matching::{CreateStudyDateRequest, StudyDateCard};
use leptos::prelude::*;
use leptos::tachys::html::event::SubmitEvent;
use leptos_router::components::A;

#[component]
pub fn StudyDateForm() -> impl IntoView {
    let (starts_at, set_starts_at) = signal(String::new());
    let (duration_hours, set_duration_hours) = signal("3".to_string());
    let (error, set_error) = signal(None::<String>);
    let (success, set_success) = signal(false);
    let (busy, set_busy) = signal(false);

    let on_submit = move |event: SubmitEvent| {
        event.prevent_default();
        set_error.set(None);
        set_success.set(false);

        let starts_at = starts_at.get_untracked();
        if starts_at.is_empty() {
            set_error.set(Some("Choose a start date and time".to_string()));
            return;
        }
        let duration_min = match duration_hours
            .get_untracked()
            .parse::<i32>()
            .ok()
            .and_then(|hours| hours.checked_mul(60))
        {
            Some(minutes) if minutes > 0 => minutes,
            _ => {
                set_error.set(Some(
                    "Enter a whole number of hours greater than zero".to_string(),
                ));
                return;
            }
        };

        set_busy.set(true);
        leptos::task::spawn_local(async move {
            let body = CreateStudyDateRequest {
                starts_at,
                duration_min,
            };
            match api::post_json::<_, StudyDateCard>("/api/study-dates", &body).await {
                Ok(_) => set_success.set(true),
                Err(message) => set_error.set(Some(message)),
            }
            set_busy.set(false);
        });
    };

    view! {
        <section aria-labelledby="study-date-title">
            <h1 id="study-date-title">"Create a study date"</h1>
            <form novalidate on:submit=on_submit>
                <label for="study-date-start">"Start date and time"</label>
                <input
                    id="study-date-start"
                    type="datetime-local"
                    required
                    bind:value=(starts_at, set_starts_at)
                />
                <label for="study-date-duration">"Duration (hours; maximum 12)"</label>
                <input
                    id="study-date-duration"
                    type="number"
                    min="1"
                    step="1"
                    required
                    bind:value=(duration_hours, set_duration_hours)
                />
                <button type="submit" disabled=move || busy.get()>
                    {move || if busy.get() { "Publishing..." } else { "Publish study date" }}
                </button>
            </form>
            {move || error.get().map(|message| view! { <p role="alert">{message}</p> })}
            {move || {
                success.get().then(|| {
                    view! {
                        <p role="status">"Study date published."</p>
                        <A href="/feed">"View study dates"</A>
                    }
                })
            }}
        </section>
    }
}
