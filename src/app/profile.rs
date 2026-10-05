use super::api;
use crate::share::matching::{display_name, ProfileView, Relation, SendMatchRequest};
use crate::share::register::MessageBody;
use leptos::prelude::*;
use leptos_router::{components::A, hooks::use_params_map};

#[component]
pub fn ProfilePage() -> impl IntoView {
    let params = use_params_map();
    let user_id = move || params.read().get("id").and_then(|s| s.parse::<i64>().ok());

    let (profile, set_profile) = signal(None::<ProfileView>);
    let (error, set_error) = signal(None::<String>);
    let (busy, set_busy) = signal(false);

    Effect::new(move |_| {
        let Some(id) = user_id() else {
            set_error.set(Some("Invalid profile".to_string()));
            return;
        };
        set_profile.set(None);
        set_error.set(None);
        leptos::task::spawn_local(async move {
            match api::get_json::<ProfileView>(&format!("/api/profiles/{id}")).await {
                Ok(p) => set_profile.set(Some(p)),
                Err(m) => set_error.set(Some(m)),
            }
        });
    });

    let connect = move |_| {
        let Some(id) = user_id() else { return };
        set_busy.set(true);
        set_error.set(None);
        leptos::task::spawn_local(async move {
            let body = SendMatchRequest { to_user_id: id };
            match api::post_json::<_, MessageBody>("/api/match-requests", &body).await {
                Ok(_) => set_profile.update(|p| {
                    if let Some(p) = p {
                        p.relation = Relation::Pending;
                    }
                }),
                Err(m) => set_error.set(Some(m)),
            }
            set_busy.set(false);
        });
    };

    view! {
        <section aria-labelledby="profile-title">
            {move || error.get().map(|m| view! { <p role="alert">{m}</p> })}
            {move || match profile.get() {
                None => {
                    if error.get().is_some() {
                        ().into_any()
                    } else {
                        view! { <p role="status">"Loading profile..."</p> }.into_any()
                    }
                }
                Some(p) => {
                    let action = match p.relation {
                        Relation::None => {
                            view! {
                                <button on:click=connect disabled=move || busy.get()>
                                    "Connect"
                                </button>
                            }
                                .into_any()
                        }
                        Relation::Pending => view! { <button disabled>"Pending"</button> }.into_any(),
                        Relation::Incoming => {
                            view! {
                                <p>
                                    "This user sent you a request. "
                                    <A href="/requests">"Review it"</A>
                                </p>
                            }
                                .into_any()
                        }
                        Relation::Matched => view! { <p>"Matched"</p> }.into_any(),
                        Relation::SelfProfile => view! { <p>"This is your profile"</p> }.into_any(),
                    };
                    view! {
                        <h1 id="profile-title">{display_name(p.user_id)}</h1>
                        {action}
                    }
                        .into_any()
                }
            }}
        </section>
    }
}
