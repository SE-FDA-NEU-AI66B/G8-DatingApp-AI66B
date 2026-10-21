pub(super) mod chat;
pub(super) mod explore;
pub(super) mod layout;
pub(super) mod requests;
pub(super) mod study_date_form;
pub(super) mod study_feed;

use super::community_feed;
use crate::share::auth::SessionInfo;
use leptos::prelude::*;
use leptos_router::components::A;

async fn load_session() -> Result<SessionInfo, String> {
    use gloo_net::http::Request;

    let response = Request::get("/api/auth/session")
        .send()
        .await
        .map_err(|_| "Unable to load your account. Please try again.".to_string())?;
    if response.ok() {
        response
            .json::<SessionInfo>()
            .await
            .map_err(|_| "The account service returned an unexpected response.".to_string())
    } else {
        Err("Your session has expired. Please sign in again.".to_string())
    }
}

#[component]
pub fn UserHome(#[prop(default = "home")] section: &'static str) -> impl IntoView {
    let (account, set_account) = signal(None::<SessionInfo>);
    let (error, set_error) = signal(None::<String>);

    Effect::new(move |_| {
        leptos::task::spawn_local(async move {
            match load_session().await {
                Ok(session) => set_account.set(Some(session)),
                Err(message) => set_error.set(Some(message)),
            }
        });
    });

    view! {
        <section class="user-home" aria-label="Your home">
            {move || match (account.get(), error.get()) {
                (Some(session), _) => view! {
                    <div class="user-home-content">
                    {match section {
                        "explore" => view! { <explore::ExplorePage /> }.into_any(),
                        "dates" => view! { <study_feed::StudyFeed /> }.into_any(),
                        _ => view! {
                    <div class="user-home-welcome">
                        <div class="user-home-welcome-copy">
                            <p class="section-kicker">"YOUR CAMPUS, YOUR PEOPLE"</p>
                            <h1 id="user-home-title">"Your home"</h1>
                        </div>
                        <span class="verified-pill"><span>"✓"</span>" NEU account"</span>
                    </div>

                    <section class="user-profile-card" aria-label="Your profile">
                        <div class="user-profile-avatar" aria-hidden="true">
                            {session.username.chars().next().unwrap_or('S').to_uppercase().to_string()}
                        </div>
                        <div class="user-profile-details">
                            <p class="section-kicker">"YOUR PROFILE"</p>
                            <h2>{session.username.clone()}</h2>
                            <p class="user-home-email">{session.email.clone()}</p>
                        </div>
                        <span class="role-label">{session.role.clone()}</span>
                    </section>

                    <div class="user-home-hero">
                        <div class="user-home-hero-copy">
                            <span class="home-sparkle">"✦"</span>
                            <p class="section-kicker">"MAKE TODAY A LITTLE MORE INTERESTING"</p>
                            <h2>"A good connection can start with one small plan."</h2>
                            <p>"Find someone to study with, grab a coffee, and see where the conversation goes."</p>
                            <A href="/userhome/study-dates" attr:class="button button-primary">"Explore study dates "<span>"↗"</span></A>
                        </div>
                        <div class="home-date-stamp"><span>"TODAY"</span><strong>"♡"</strong><small>"could be the day"</small></div>
                    </div>

                    <div class="user-home-section-heading">
                        <div><p class="section-kicker">"YOUR NEXT MOVE"</p><h2>"Make a little room for connection."</h2></div>
                        <span class="role-label">"NEU student"</span>
                    </div>
                    <div class="user-home-actions">
                        <A href="/user_profile" attr:class="home-action-card action-create">
                            <span class="action-icon">"✎"</span>
                            <span class="action-card-copy"><strong>"Complete your profile"</strong><small>"Add a bio, cohort, and interests."</small></span>
                            <span class="action-arrow">"↗"</span>
                        </A>
                        <A href="/userhome/study-date/new" attr:class="home-action-card action-create">
                            <span class="action-icon">"＋"</span>
                            <span class="action-card-copy"><strong>"Post a study date"</strong><small>"Set a time and invite someone along."</small></span>
                            <span class="action-arrow">"↗"</span>
                        </A>
                        <A href="/userhome/requests" attr:class="home-action-card action-requests">
                            <span class="action-icon">"♡"</span>
                            <span class="action-card-copy"><strong>"Check your requests"</strong><small>"See who would like to meet."</small></span>
                            <span class="action-arrow">"↗"</span>
                        </A>
                    </div>
                    <community_feed::CommunityFeed />
                        }.into_any(),
                    }}
                    </div>
                }.into_any(),
                (None, Some(message)) => view! {
                    <div class="user-home-message" role="alert">
                        <h1 id="user-home-title">"We couldn't load your home."</h1>
                        <p>{message}</p>
                        <A href="/login" attr:class="button button-primary">"Sign in again"</A>
                    </div>
                }.into_any(),
                (None, None) => view! {
                    <div class="user-home-message" role="status">
                        <span class="loading-heart">"♡"</span>
                        <p id="user-home-title">"Getting your home ready…"</p>
                    </div>
                }.into_any(),
            }}
        </section>
    }
}
