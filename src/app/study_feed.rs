use super::api;
use crate::share::matching::{display_name, StudyDateCard};
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn StudyFeed() -> impl IntoView {
    let (cards, set_cards) = signal(None::<Vec<StudyDateCard>>);
    let (error, set_error) = signal(None::<String>);

    Effect::new(move |_| {
        leptos::task::spawn_local(async move {
            match api::get_json::<Vec<StudyDateCard>>("/api/study-dates").await {
                Ok(list) => set_cards.set(Some(list)),
                Err(message) => set_error.set(Some(message)),
            }
        });
    });

    view! {
        <section aria-labelledby="feed-title">
            <h1 id="feed-title">"Study dates"</h1>
            {move || {
                if let Some(message) = error.get() {
                    return view! { <p role="alert">{message}</p> }.into_any();
                }
                match cards.get() {
                    None => view! { <p role="status">"Loading study dates..."</p> }.into_any(),
                    Some(list) if list.is_empty() => {
                        view! { <p role="status">"0 study dates found for today"</p> }.into_any()
                    }
                    Some(list) => {
                        view! {
                            <ul>
                                {list
                                    .into_iter()
                                    .map(|card| {
                                        let who = if card.is_mine {
                                            view! { <span>"You"</span> }.into_any()
                                        } else {
                                            view! {
                                                <A href=format!("/profile/{}", card.creator_id)>
                                                    {display_name(card.creator_id)}
                                                </A>
                                            }
                                                .into_any()
                                        };
                                        view! {
                                            <li>
                                                <article class="study-card">
                                                    <h2>{card.starts_at} " → " {card.ends_at}</h2>
                                                    <p>{who}</p>
                                                </article>
                                            </li>
                                        }
                                    })
                                    .collect_view()}
                            </ul>
                        }
                            .into_any()
                    }
                }
            }}
        </section>
    }
}
