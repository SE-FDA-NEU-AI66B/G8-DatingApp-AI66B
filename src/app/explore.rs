use super::api;
use crate::share::{
    matching::{ProfileSearchResult, SendMatchRequest},
    register::MessageBody,
};
use leptos::prelude::*;
use leptos_router::components::A;
use uuid::Uuid;

#[component]
pub fn ExplorePage() -> impl IntoView {
    let (profiles, set_profiles) = signal(None::<Vec<ProfileSearchResult>>);
    let (error, set_error) = signal(None::<String>);
    let (feedback, set_feedback) = signal(None::<String>);
    let (sending, set_sending) = signal(None::<Uuid>);
    let (sent, set_sent) = signal(Vec::<Uuid>::new());

    Effect::new(move |_| {
        leptos::task::spawn_local(async move {
            match api::get_json::<Vec<ProfileSearchResult>>("/api/profiles/search?q=").await {
                Ok(items) => set_profiles.set(Some(items)),
                Err(message) => set_error.set(Some(message)),
            }
        });
    });

    let send_request = move |user_id: Uuid| {
        if sending.get_untracked().is_some() || sent.get_untracked().contains(&user_id) {
            return;
        }
        set_sending.set(Some(user_id));
        set_feedback.set(None);
        leptos::task::spawn_local(async move {
            match api::post_json::<_, MessageBody>(
                "/api/match-requests",
                &SendMatchRequest {
                    to_user_id: user_id,
                },
            )
            .await
            {
                Ok(_) => {
                    set_sent.update(|items| items.push(user_id));
                    set_feedback.set(Some("Request sent".to_string()));
                }
                Err(message) => set_feedback.set(Some(message)),
            }
            set_sending.set(None);
        });
    };

    view! {
        <section class="potential-match-feed" aria-labelledby="explore-title">
            <header class="potential-match-heading">
                <p class="section-kicker">"MEET SOMEONE NEW"</p>
                <h1 id="explore-title">"Explore people"</h1>
                <p>"Find potential study partners and get to know a little about them."</p>
            </header>
            {move || error.get().map(|message| view! { <p role="alert">{message}</p> })}
            {move || feedback.get().map(|message| view! { <p class="potential-match-feedback" role="status">{message}</p> })}
            {move || match profiles.get() {
                None => view! { <p role="status">"Finding potential matches…"</p> }.into_any(),
                Some(items) if items.is_empty() => view! {
                    <div class="potential-match-empty"><h2>"No potential matches yet"</h2><p>"Check back later to meet more NEU students."</p></div>
                }.into_any(),
                Some(items) => view! {
                    <div class="potential-match-grid">
                        {items.into_iter().map(|profile| {
                            let user_id = profile.user_id;
                            let is_sent = move || sent.get().contains(&user_id);
                            let is_sending = move || sending.get() == Some(user_id);
                            view! {
                                <article class="potential-match-card">
                                    <div class="potential-match-avatar" aria-hidden="true">
                                        {profile.display_name.chars().next().unwrap_or('S').to_uppercase().to_string()}
                                    </div>
                                    <p class="potential-match-cohort">{profile.cohort.clone().unwrap_or_else(|| "NEU student".to_string())}</p>
                                    <h2><A href=format!("/profile/{user_id}")>{profile.display_name}</A></h2>
                                    {if profile.bio.is_empty() {
                                        view! { <p class="potential-match-bio">"This student hasn’t added a bio yet."</p> }.into_any()
                                    } else {
                                        view! { <p class="potential-match-bio">{profile.bio}</p> }.into_any()
                                    }}
                                    {(!profile.interests.is_empty()).then(|| view! {
                                        <p class="potential-match-interests"><strong>"Interests"</strong><span>{profile.interests}</span></p>
                                    })}
                                    <div class="potential-match-actions">
                                        <A href=format!("/profile/{user_id}") attr:class="potential-match-view">"View profile"</A>
                                        <button type="button" disabled=move || is_sent() || is_sending()
                                            on:click=move |_| send_request(user_id)>
                                            {move || if is_sent() { "Request sent" } else if is_sending() { "Sending…" } else { "Connect" }}
                                        </button>
                                    </div>
                                </article>
                            }
                        }).collect_view()}
                    </div>
                }.into_any(),
            }}
        </section>
    }
}
