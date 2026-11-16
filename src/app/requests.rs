use super::api;
use crate::share::matching::{display_name, IncomingRequest, MatchedUser};
use crate::share::register::MessageBody;
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn RequestsPage() -> impl IntoView {
    let (incoming, set_incoming) = signal(None::<Vec<IncomingRequest>>);
    let (matched, set_matched) = signal(Vec::<MatchedUser>::new());
    let (error, set_error) = signal(None::<String>);

    Effect::new(move |_| {
        leptos::task::spawn_local(async move {
            match api::get_json::<Vec<IncomingRequest>>("/api/match-requests/incoming").await {
                Ok(list) => set_incoming.set(Some(list)),
                Err(m) => set_error.set(Some(m)),
            }
        });
        leptos::task::spawn_local(async move {
            if let Ok(list) = api::get_json::<Vec<MatchedUser>>("/api/matches").await {
                set_matched.set(list);
            }
        });
    });

    let remove = move |id: i64| {
        set_incoming.update(|l| {
            if let Some(l) = l {
                l.retain(|r| r.id != id);
            }
        });
    };

    let respond = move |id: i64, accept: bool| {
        set_error.set(None);
        leptos::task::spawn_local(async move {
            if accept {
                let url = format!("/api/match-requests/{id}/accept");
                match api::post_empty::<MatchedUser>(&url).await {
                    Ok(m) => {
                        remove(id);
                        set_matched.update(|v| v.insert(0, m));
                    }
                    Err(m) => set_error.set(Some(m)),
                }
            } else {
                let url = format!("/api/match-requests/{id}/decline");
                match api::post_empty::<MessageBody>(&url).await {
                    Ok(_) => remove(id),
                    Err(m) => set_error.set(Some(m)),
                }
            }
        });
    };

    view! {
        <section aria-labelledby="requests-title">
            <h1 id="requests-title">"Match requests"</h1>
            {move || error.get().map(|m| view! { <p role="alert">{m}</p> })}
            <h2>
                {move || {
                    format!("Pending requests ({})", incoming.get().map_or(0, |l| l.len()))
                }}
            </h2>
            {move || match incoming.get() {
                None => view! { <p role="status">"Loading requests..."</p> }.into_any(),
                Some(list) if list.is_empty() => {
                    view! { <p role="status">"No pending requests"</p> }.into_any()
                }
                Some(list) => {
                    view! {
                        <ul>
                            {list
                                .into_iter()
                                .map(|r| {
                                    let id = r.id;
                                    view! {
                                        <li>
                                            <A href=format!("/profile/{}", r.sender_id)>
                                                {display_name(r.sender_id)}
                                            </A>
                                            " · " {r.created_at}
                                            <button on:click=move |_| respond(id, true)>"Accept"</button>
                                            <button on:click=move |_| respond(id, false)>"Decline"</button>
                                        </li>
                                    }
                                })
                                .collect_view()}
                        </ul>
                    }
                        .into_any()
                }
            }}
            <h2>"Matched"</h2>
            <ul>
                {move || {
                    matched
                        .get()
                        .into_iter()
                        .map(|m| {
                            view! {
                                <li>
                                    {display_name(m.user_id)} " · chat room #" {m.chat_room_id}
                                </li>
                            }
                        })
                        .collect_view()
                }}
            </ul>
        </section>
    }
}
