use super::api;
use crate::share::admin::ReportSummary;
use crate::share::register::MessageBody;
use leptos::prelude::*;

#[component]
pub fn AdminReportsPage() -> impl IntoView {
    let (reports, set_reports) = signal(Vec::<ReportSummary>::new());
    let (error, set_error) = signal(None::<String>);
    #[cfg(not(target_arch = "wasm32"))]
    let _ = &set_error;

    #[cfg(target_arch = "wasm32")]
    leptos::task::spawn_local(async move {
        match api::get_json::<Vec<ReportSummary>>("/api/admin/reports").await {
            Ok(value) => set_reports.set(value),
            Err(message) => set_error.set(Some(message)),
        }
    });

    view! {
        <section aria-labelledby="admin-reports-title">
            <h1 id="admin-reports-title">"Reported profiles"</h1>
            {move || error.get().map(|message| view! { <p role="alert">{message}</p> })}
            <div class="admin-report-list">
                {move || reports.get().into_iter().map(|report| {
                    let report_id = report.id;
                    view! {
                        <article>
                            <h2>{format!("Profile {}", report.reported_user_id)}</h2>
                            <p>{format!("{} reports in the last 24 hours", report.report_count)}</p>
                            <p>{report.reason}</p>
                            <button on:click=move |_| {
                                leptos::task::spawn_local(async move {
                                    if api::post_empty::<MessageBody>(
                                        &format!("/api/admin/reports/{report_id}/hide"),
                                    ).await.is_ok() {
                                        set_reports.update(|items| items.retain(|item| item.id != report_id));
                                    }
                                });
                            }>"Hide profile"</button>
                            <button on:click=move |_| {
                                leptos::task::spawn_local(async move {
                                    if api::post_empty::<MessageBody>(
                                        &format!("/api/admin/reports/{report_id}/dismiss"),
                                    ).await.is_ok() {
                                        set_reports.update(|items| items.retain(|item| item.id != report_id));
                                    }
                                });
                            }>"Dismiss"</button>
                        </article>
                    }
                }).collect_view()}
            </div>
        </section>
    }
}
