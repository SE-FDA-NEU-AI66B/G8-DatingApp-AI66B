#[cfg(target_arch = "wasm32")]
use crate::share::admin::StatisticsError;
use crate::share::admin::{DashboardStatistics, HealthStatus};
use leptos::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
struct DashboardError {
    status: Option<u16>,
    message: String,
}

fn error_message(error: &DashboardError) -> String {
    match error.status {
        Some(401) => "Your session has expired. Sign in again to continue.".to_string(),
        Some(403) => "You do not have permission to view this dashboard.".to_string(),
        _ => error.message.clone(),
    }
}

#[cfg(target_arch = "wasm32")]
async fn fetch_statistics() -> Result<DashboardStatistics, DashboardError> {
    use gloo_net::http::Request;

    let response = Request::get("/api/admin/statistics")
        .send()
        .await
        .map_err(|_| DashboardError {
            status: None,
            message: "Unable to reach the dashboard statistics service.".to_string(),
        })?;

    if response.ok() {
        return response.json().await.map_err(|_| DashboardError {
            status: Some(response.status()),
            message: "The dashboard returned an unexpected response.".to_string(),
        });
    } else {
        let error = response.json::<StatisticsError>().await.ok();
        Err(DashboardError {
            status: Some(response.status()),
            message: error.map_or_else(
                || "Dashboard statistics are temporarily unavailable.".to_string(),
                |error| error.message,
            ),
        })
    }
}

#[cfg(target_arch = "wasm32")]
async fn fetch_health() -> Result<HealthStatus, String> {
    use gloo_net::http::Request;

    let response = Request::get("/api/admin/health")
        .send()
        .await
        .map_err(|_| "Unable to reach the health service.".to_string())?;

    if response.ok() {
        response
            .json()
            .await
            .map_err(|_| "The health service returned an unexpected response.".to_string())
    } else {
        Err("Health status is unavailable.".to_string())
    }
}

#[component]
pub fn AdminDashboard() -> impl IntoView {
    let (statistics, set_statistics) = signal(None::<DashboardStatistics>);
    let (health, set_health) = signal(None::<HealthStatus>);
    let (error, set_error) = signal(None::<DashboardError>);
    let (health_error, set_health_error) = signal(None::<String>);
    let (loading, set_loading) = signal(false);

    #[cfg(not(target_arch = "wasm32"))]
    let _ = (&set_statistics, &set_health);

    let load = move || {
        if loading.get_untracked() {
            return;
        }
        set_loading.set(true);
        set_error.set(None);
        set_health_error.set(None);

        #[cfg(target_arch = "wasm32")]
        {
            leptos::task::spawn_local(async move {
                match fetch_statistics().await {
                    Ok(value) => set_statistics.set(Some(value)),
                    Err(error) => {
                        set_error.set(Some(error));
                    }
                }
                set_loading.set(false);
            });
            leptos::task::spawn_local(async move {
                match fetch_health().await {
                    Ok(value) => set_health.set(Some(value)),
                    Err(message) => {
                        set_health.set(None);
                        set_health_error.set(Some(message));
                    }
                }
            });
        }
    };

    #[cfg(target_arch = "wasm32")]
    {
        load();
    }

    view! {
        <section
            aria-labelledby="admin-dashboard-title"
            aria-busy=move || loading.get()
            class="admin-dashboard"
        >
            <h1 id="admin-dashboard-title">"Admin dashboard"</h1>
            <button
                type="button"
                aria-label="Refresh dashboard statistics"
                on:click=move |_| load()
                disabled=move || loading.get()
            >
                {move || if loading.get() { "Refreshing..." } else { "Refresh" }}
            </button>
            {move || {
                if loading.get() && statistics.get().is_none() {
                    view! { <p role="status" aria-live="polite">"Loading dashboard statistics..."</p> }.into_any()
                } else if let Some(value) = statistics.get() {
                    view! {
                        <div class="dashboard-statistics">
                            <article>
                                <h2>"Registered users"</h2>
                                <p>{value.total_registered_users}</p>
                            </article>
                            <article>
                                <h2>"Active study dates"</h2>
                                <p>{value.active_study_date_requests}</p>
                            </article>
                            <article>
                                <h2>"Pending reports"</h2>
                                <p>{value.pending_reports}</p>
                            </article>
                        </div>
                        {error.get().map(|error| view! {
                            <p role="alert" aria-live="assertive">{error_message(&error)}</p>
                        })}
                        {if value.total_registered_users == 0
                            && value.active_study_date_requests == 0
                            && value.pending_reports == 0 {
                            view! {
                                <p role="status">
                                    "No dashboard activity yet. Invite verified users or review new activity when it arrives."
                                </p>
                            }.into_any()
                        } else {
                            view! {}.into_any()
                        }}
                    }.into_any()
                } else if let Some(error) = error.get() {
                    view! {
                        <p role="alert" aria-live="assertive">{error_message(&error)}</p>
                        {if matches!(error.status, Some(401 | 403)) {
                                    view! {
                                        <a href="/">"Return to the home page"</a>
                                    }.into_any()
                        } else {
                            view! {
                                <button type="button" on:click=move |_| load() disabled=move || loading.get()>
                                    "Try again"
                                </button>
                            }.into_any()
                        }}
                    }.into_any()
                } else {
                    view! { <p role="alert" aria-live="assertive">"Dashboard statistics are unavailable."</p> }.into_any()
                }
            }}
            {move || {
                if let Some(message) = health_error.get() {
                    view! { <p role="alert" aria-live="assertive">{message}</p> }.into_any()
                } else if let Some(value) = health.get() {
                    view! {
                        <section aria-labelledby="health-status-title">
                            <h2 id="health-status-title">"System health"</h2>
                            <p class:health-unhealthy=move || !value.server.healthy>
                                <span class="health-label">"Server: "</span>
                                <span aria-hidden="true">{if value.server.healthy { "●" } else { "!" }}</span>
                                " "
                                {if value.server.healthy { "Healthy" } else { "Unhealthy" }}
                                " (last checked " {value.server.checked_at.clone()} ")"
                            </p>
                            <p class:health-unhealthy=move || !value.database.healthy>
                                <span class="health-label">"Database: "</span>
                                <span aria-hidden="true">{if value.database.healthy { "●" } else { "!" }}</span>
                                " "
                                {if value.database.healthy { "Healthy" } else { "Unhealthy" }}
                                " (last checked " {value.database.checked_at.clone()} ")"
                            </p>
                        </section>
                    }.into_any()
                } else {
                    view! { <p role="status">"Checking system health..."</p> }.into_any()
                }
            }}
        </section>
    }
}
