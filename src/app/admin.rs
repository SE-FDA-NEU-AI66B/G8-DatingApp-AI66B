use crate::share::admin::DashboardStatistics;
#[cfg(target_arch = "wasm32")]
use crate::share::admin::StatisticsError;
use leptos::prelude::*;

#[cfg(target_arch = "wasm32")]
async fn fetch_statistics() -> Result<DashboardStatistics, String> {
    use gloo_net::http::Request;

    let response = Request::get("/api/admin/statistics")
        .send()
        .await
        .map_err(|_| "Unable to reach the dashboard statistics service.".to_string())?;

    if response.ok() {
        response
            .json()
            .await
            .map_err(|_| "The dashboard returned an unexpected response.".to_string())
    } else {
        let error = response.json::<StatisticsError>().await.ok();
        Err(error.map_or_else(
            || "Dashboard statistics are temporarily unavailable.".to_string(),
            |error| error.message,
        ))
    }
}

#[component]
pub fn AdminDashboard() -> impl IntoView {
    let (statistics, set_statistics) = signal(None::<DashboardStatistics>);
    let (error, set_error) = signal(None::<String>);
    let (loading, set_loading) = signal(true);

    #[cfg(not(target_arch = "wasm32"))]
    let _ = &set_statistics;

    let load = move || {
        set_loading.set(true);
        set_error.set(None);

        #[cfg(target_arch = "wasm32")]
        {
            leptos::task::spawn_local(async move {
                match fetch_statistics().await {
                    Ok(value) => set_statistics.set(Some(value)),
                    Err(message) => {
                        set_statistics.set(None);
                        set_error.set(Some(message));
                    }
                }
                set_loading.set(false);
            });
        }
    };

    #[cfg(target_arch = "wasm32")]
    {
        load();
    }

    view! {
        <section aria-labelledby="admin-dashboard-title">
            <h1 id="admin-dashboard-title">"Admin dashboard"</h1>
            <button type="button" on:click=move |_| load() disabled=move || loading.get()>
                {move || if loading.get() { "Refreshing..." } else { "Refresh" }}
            </button>
            {move || {
                if loading.get() && statistics.get().is_none() {
                    view! { <p role="status">"Loading dashboard statistics..."</p> }.into_any()
                } else if let Some(message) = error.get() {
                    view! {
                        <p role="alert">{message}</p>
                        <button type="button" on:click=move |_| load()>"Try again"</button>
                    }.into_any()
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
                    }.into_any()
                } else {
                    view! { <p role="alert">"Dashboard statistics are unavailable."</p> }.into_any()
                }
            }}
        </section>
    }
}
