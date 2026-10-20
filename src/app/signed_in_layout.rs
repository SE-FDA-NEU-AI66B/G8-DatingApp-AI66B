use super::api;
use crate::share::{
    chat::ConversationCard,
    matching::{
        display_name, IncomingRequest, MatchedUser, ProfileSearchResult, SendMatchRequest,
        StudyDateCard,
    },
    register::MessageBody,
};
use leptos::prelude::*;
use leptos::serde_json;
use leptos_router::{components::A, hooks::use_location};

#[derive(Clone, Copy, PartialEq, Eq)]
enum SidePanel {
    Chat,
    Dates,
    Requests,
}

fn load_pins(key: &str) -> Vec<i64> {
    #[cfg(target_arch = "wasm32")]
    {
        return web_sys::window()
            .and_then(|window| window.local_storage().ok().flatten())
            .and_then(|storage| storage.get_item(key).ok().flatten())
            .and_then(|value| serde_json::from_str(&value).ok())
            .unwrap_or_default();
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = key;
        Vec::new()
    }
}

fn save_pins(key: &str, pins: &[i64]) {
    #[cfg(target_arch = "wasm32")]
    if let Some(storage) =
        web_sys::window().and_then(|window| window.local_storage().ok().flatten())
    {
        if let Ok(serialized) = serde_json::to_string(pins) {
            let _ = storage.set_item(key, &serialized);
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    let _ = (key, pins);
}

fn move_pin(pins: &mut [i64], id: i64, offset: isize) {
    let Some(index) = pins.iter().position(|pin| *pin == id) else {
        return;
    };
    let target = index as isize + offset;
    if target >= 0 && (target as usize) < pins.len() {
        pins.swap(index, target as usize);
    }
}

#[component]
pub fn SignedInLayout(children: ChildrenFn) -> impl IntoView {
    let location = use_location();
    let pathname = location.pathname;
    let (expanded, set_expanded) = signal(None::<SidePanel>);
    let (conversations, set_conversations) = signal(None::<Vec<ConversationCard>>);
    let (dates, set_dates) = signal(None::<Vec<StudyDateCard>>);
    let (requests, set_requests) = signal(None::<Vec<IncomingRequest>>);
    let (explore_active, set_explore_active) = signal(true);
    let (search_text, set_search_text) = signal(String::new());
    let (search_results, set_search_results) = signal(None::<Vec<ProfileSearchResult>>);
    let (search_generation, set_search_generation) = signal(0_u64);
    let (request_feedback, set_request_feedback) = signal(None::<String>);
    let (pinned_chats, set_pinned_chats) = signal(load_pins("pinned-chats"));
    let (pinned_dates, set_pinned_dates) = signal(load_pins("pinned-study-dates"));
    let (panel_error, set_panel_error) = signal(None::<String>);

    let open_chat = move |_| {
        if expanded.get_untracked() == Some(SidePanel::Chat) {
            set_expanded.set(None);
            return;
        }
        set_expanded.set(Some(SidePanel::Chat));
        if conversations.get_untracked().is_none() {
            leptos::task::spawn_local(async move {
                match api::get_json::<Vec<ConversationCard>>("/api/conversations").await {
                    Ok(items) => set_conversations.set(Some(items)),
                    Err(error) => set_panel_error.set(Some(error)),
                }
            });
        }
    };
    let open_dates = move |_| {
        if expanded.get_untracked() == Some(SidePanel::Dates) {
            set_expanded.set(None);
            return;
        }
        set_expanded.set(Some(SidePanel::Dates));
        if dates.get_untracked().is_none() {
            leptos::task::spawn_local(async move {
                match api::get_json::<Vec<StudyDateCard>>("/api/study-dates").await {
                    Ok(items) => set_dates.set(Some(items)),
                    Err(error) => set_panel_error.set(Some(error)),
                }
            });
        }
    };
    let open_requests = move |_| {
        if expanded.get_untracked() == Some(SidePanel::Requests) {
            set_expanded.set(None);
            return;
        }
        set_expanded.set(Some(SidePanel::Requests));
        set_explore_active.set(false);
        if requests.get_untracked().is_none() {
            leptos::task::spawn_local(async move {
                match api::get_json::<Vec<IncomingRequest>>("/api/match-requests/incoming").await {
                    Ok(items) => set_requests.set(Some(items)),
                    Err(error) => set_panel_error.set(Some(error)),
                }
            });
        }
        if search_results.get_untracked().is_none() {
            leptos::task::spawn_local(async move {
                match api::get_json::<Vec<ProfileSearchResult>>("/api/profiles/search?q=").await {
                    Ok(items) => set_search_results.set(Some(items)),
                    Err(error) => set_panel_error.set(Some(error)),
                }
            });
        }
    };
    let respond_to_request = move |id: i64, accept: bool| {
        set_panel_error.set(None);
        leptos::task::spawn_local(async move {
            if accept {
                match api::post_empty::<MatchedUser>(&format!("/api/match-requests/{id}/accept"))
                    .await
                {
                    Ok(_) => {
                        set_requests.update(|items| {
                            if let Some(items) = items {
                                items.retain(|item| item.id != id);
                            }
                        });
                        match api::get_json::<Vec<ConversationCard>>("/api/conversations").await {
                            Ok(items) => set_conversations.set(Some(items)),
                            Err(error) => set_panel_error.set(Some(error)),
                        }
                    }
                    Err(error) => set_panel_error.set(Some(error)),
                }
            } else {
                match api::post_empty::<MessageBody>(&format!("/api/match-requests/{id}/decline"))
                    .await
                {
                    Ok(_) => set_requests.update(|items| {
                        if let Some(items) = items {
                            if let Some(item) = items.iter_mut().find(|item| item.id == id) {
                                item.status = "declined".to_string();
                            }
                        }
                    }),
                    Err(error) => set_panel_error.set(Some(error)),
                }
            }
        });
    };
    let create_request = move |user_id: uuid::Uuid| {
        set_request_feedback.set(None);
        leptos::task::spawn_local(async move {
            match api::post_json::<_, MessageBody>(
                "/api/match-requests",
                &SendMatchRequest {
                    to_user_id: user_id,
                },
            )
            .await
            {
                Ok(_) => set_request_feedback.set(Some("Request sent".to_string())),
                Err(error) => set_request_feedback.set(Some(error)),
            }
        });
    };

    Effect::new(move |_| {
        let pins = pinned_chats.get();
        save_pins("pinned-chats", &pins);
    });
    Effect::new(move |_| {
        let pins = pinned_dates.get();
        save_pins("pinned-study-dates", &pins);
    });

    view! {
        <div class=move || if expanded.get().is_some() { "signed-in-layout has-panel" } else { "signed-in-layout" }>
            <aside class="signed-in-sidebar"
                aria-label="Signed-in navigation">
                <nav class="signed-in-nav" aria-label="Workspace">
                    <A href="/userhome" attr:class=move || if expanded.get().is_none() && pathname.get() == "/userhome" { "signed-in-nav-link is-active" } else { "signed-in-nav-link" } attr:title="Home" attr:aria-label="Home" on:click=move |_| set_expanded.set(None)>
                        <span aria-hidden="true">"⌂"</span><span class="signed-in-nav-label">"Home"</span>
                    </A>
                    <A href="/explore" attr:class=move || if expanded.get().is_none() && pathname.get() == "/explore" { "signed-in-nav-link is-active" } else { "signed-in-nav-link" } attr:title="Explore" attr:aria-label="Explore potential matches" on:click=move |_| set_expanded.set(None)>
                        <span aria-hidden="true">"⌕"</span><span class="signed-in-nav-label">"Explore"</span>
                    </A>
                    <button type="button" class="signed-in-nav-link" class:panel-active=move || if let Some(panel) = expanded.get() { panel == SidePanel::Chat } else { pathname.get().starts_with("/chat") } title="Chat" aria-label="Chat"
                        aria-expanded=move || expanded.get() == Some(SidePanel::Chat) on:click=open_chat>
                        <span aria-hidden="true">"✉"</span>
                    </button>
                    <button type="button" class="signed-in-nav-link" class:panel-active=move || if let Some(panel) = expanded.get() { panel == SidePanel::Dates } else { pathname.get().starts_with("/study-date") || pathname.get() == "/feed" } title="Study date" aria-label="Study date"
                        aria-expanded=move || expanded.get() == Some(SidePanel::Dates) on:click=open_dates>
                        <span aria-hidden="true">"＋"</span>
                    </button>
                    <button type="button" class="signed-in-nav-link" class:panel-active=move || if let Some(panel) = expanded.get() { panel == SidePanel::Requests } else { pathname.get() == "/requests" } title="Requests" aria-label="Requests"
                        aria-expanded=move || expanded.get() == Some(SidePanel::Requests) on:click=open_requests>
                        <span aria-hidden="true">"♡"</span>
                    </button>
                </nav>
            </aside>

            <Show when=move || expanded.get().is_some()>
                <aside class="signed-in-panel" aria-label=move || match expanded.get() {
                    Some(SidePanel::Chat) => "Chat list",
                    Some(SidePanel::Dates) => "Study dates",
                    Some(SidePanel::Requests) => "Match requests",
                    None => "Workspace panel",
                }>
                    <Show when=move || expanded.get() == Some(SidePanel::Chat) fallback=|| ()>
                        <div class="signed-in-panel-heading"><h2>"Chat"</h2></div>
                        {move || panel_error.get().map(|error| view! { <p role="alert">{error}</p> })}
                        {move || match conversations.get() {
                            None => view! { <p role="status">"Loading chats…"</p> }.into_any(),
                            Some(items) if items.is_empty() => view! { <p class="signed-in-panel-empty">"No chats yet"</p> }.into_any(),
                            Some(mut items) => {
                                let pins = pinned_chats.get();
                                items.sort_by_key(|item| {
                                    pins.iter().position(|pin| *pin == item.conversation_id)
                                        .map_or((true, usize::MAX), |index| (false, index))
                                });
                                view! { <ul class="signed-in-panel-list">{items.into_iter().map(|item| {
                                    let id = item.conversation_id;
                                    let pins = pinned_chats.get();
                                    let pin_position = pins.iter().position(|pin| *pin == id);
                                    let is_pinned = pin_position.is_some();
                                    let can_move_up = pin_position.is_some_and(|position| position > 0);
                                    let can_move_down = pin_position.is_some_and(|position| position + 1 < pins.len());
                                    view! { <li class="signed-in-panel-row">
                                        <div class="signed-in-chat-person">
                                            <A href=format!("/chat/{id}") attr:class="signed-in-panel-link signed-in-chat-profile">
                                                <span class="signed-in-panel-avatar">{item.display_name.chars().next().unwrap_or('S').to_uppercase().to_string()}</span>
                                                <strong>{item.display_name}</strong>
                                            </A>
                                            <A href=format!("/chat/{id}") attr:class="signed-in-chat-open">{item.last_message.unwrap_or_else(|| "Open conversation".into())}</A>
                                        </div>
                                        <div class="panel-item-actions">
                                            {if is_pinned { view! { <div class="panel-reorder">
                                                <button type="button" disabled=!can_move_up aria-label="Move pinned chat up" on:click=move |_| set_pinned_chats.update(|pins| move_pin(pins, id, -1))>"↑"</button>
                                                <button type="button" disabled=!can_move_down aria-label="Move pinned chat down" on:click=move |_| set_pinned_chats.update(|pins| move_pin(pins, id, 1))>"↓"</button>
                                            </div> }.into_any() } else { ().into_any() }}
                                            <button type="button" class="panel-pin" aria-label=if is_pinned { "Unpin chat" } else { "Pin chat" } aria-pressed=is_pinned
                                                on:click=move |_| set_pinned_chats.update(|pins| if let Some(index) = pins.iter().position(|pin| *pin == id) { pins.remove(index); } else { pins.push(id); })>
                                                {if is_pinned { "📌" } else { "⌖" }}
                                            </button>
                                        </div>
                                    </li> }
                                }).collect_view()}</ul> }.into_any()
                            }
                        }}
                    </Show>
                    <Show when=move || expanded.get() == Some(SidePanel::Dates) fallback=|| ()>
                        <div class="signed-in-panel-heading signed-in-date-heading">
                            <h2>"Study dates"</h2>
                            <A href="/study-date/new" attr:class="signed-in-create-date">"＋ Create new date"</A>
                        </div>
                        {move || panel_error.get().map(|error| view! { <p role="alert">{error}</p> })}
                        {move || match dates.get() {
                            None => view! { <p role="status">"Loading study dates…"</p> }.into_any(),
                            Some(items) if items.is_empty() => view! { <p class="signed-in-panel-empty">"No upcoming study dates"</p> }.into_any(),
                            Some(mut items) => {
                                let pins = pinned_dates.get();
                                items.sort_by_key(|item| {
                                    pins.iter().position(|pin| *pin == item.id)
                                        .map_or((true, usize::MAX), |index| (false, index))
                                });
                                view! { <ul class="signed-in-panel-list">{items.into_iter().map(|item| {
                                let id = item.id;
                                let pins = pinned_dates.get();
                                let pin_position = pins.iter().position(|pin| *pin == id);
                                let is_pinned = pin_position.is_some();
                                let can_move_up = pin_position.is_some_and(|position| position > 0);
                                let can_move_down = pin_position.is_some_and(|position| position + 1 < pins.len());
                                view! { <li class="signed-in-panel-row">
                                    <A href="/feed" attr:class="signed-in-panel-link signed-in-date-link">
                                        <strong>{item.starts_at}</strong><small>{if item.is_mine { "Your study date".to_string() } else { "Open study date feed".to_string() }}</small>
                                    </A>
                                    <div class="panel-item-actions">
                                        {if is_pinned { view! { <div class="panel-reorder">
                                            <button type="button" disabled=!can_move_up aria-label="Move pinned study date up" on:click=move |_| set_pinned_dates.update(|pins| move_pin(pins, id, -1))>"↑"</button>
                                            <button type="button" disabled=!can_move_down aria-label="Move pinned study date down" on:click=move |_| set_pinned_dates.update(|pins| move_pin(pins, id, 1))>"↓"</button>
                                        </div> }.into_any() } else { ().into_any() }}
                                        <button type="button" class="panel-pin" aria-label=if is_pinned { "Unpin study date" } else { "Pin study date" } aria-pressed=is_pinned
                                            on:click=move |_| set_pinned_dates.update(|pins| if let Some(index) = pins.iter().position(|pin| *pin == id) { pins.remove(index); } else { pins.push(id); })>
                                            {if is_pinned { "📌" } else { "⌖" }}
                                        </button>
                                    </div>
                                </li> }
                            }).collect_view()}</ul> }.into_any()
                            }
                        }}
                    </Show>
                    <Show when=move || expanded.get() == Some(SidePanel::Requests) fallback=|| ()>
                        <div class="signed-in-panel-heading">
                            <h2>"People"</h2>
                            <div class="signed-in-panel-tabs" role="tablist" aria-label="People">
                                <button type="button" role="tab" aria-selected=move || explore_active.get() class=move || if explore_active.get() { "is-active" } else { "" } on:click=move |_| set_explore_active.set(true)>"Explore"</button>
                                <button type="button" role="tab" aria-selected=move || !explore_active.get() class=move || if !explore_active.get() { "is-active" } else { "" } on:click=move |_| set_explore_active.set(false)>"Requests"</button>
                            </div>
                        </div>
                        <Show when=move || explore_active.get() fallback=|| ()>
                        <label class="signed-in-search-label" for="profile-search">"Find students"</label>
                        <input id="profile-search" class="signed-in-search-input" type="search" placeholder="Search name or username…"
                            prop:value=move || search_text.get()
                            on:input=move |ev| {
                                let value = event_target_value(&ev);
                                set_search_text.set(value.clone());
                                set_request_feedback.set(None);
                                set_panel_error.set(None);
                                let generation = search_generation.get_untracked() + 1;
                                set_search_generation.set(generation);
                                let encoded: String = value.trim().bytes().map(|byte| {
                                    if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~') {
                                        (byte as char).to_string()
                                    } else {
                                        format!("%{byte:02X}")
                                    }
                                }).collect();
                                set_search_results.set(None);
                                leptos::task::spawn_local(async move {
                                    match api::get_json::<Vec<ProfileSearchResult>>(&format!("/api/profiles/search?q={encoded}")).await {
                                        Ok(results) if search_generation.get_untracked() == generation => set_search_results.set(Some(results)),
                                        Err(error) if search_generation.get_untracked() == generation => set_panel_error.set(Some(error)),
                                        _ => (),
                                    }
                                });
                            } />
                        {move || request_feedback.get().map(|message| view! { <p class="signed-in-request-feedback" role="status">{message}</p> })}
                        {move || match search_results.get() {
                            None => view! { <p role="status">"Searching profiles…"</p> }.into_any(),
                            Some(results) if results.is_empty() => view! { <p class="signed-in-panel-empty">"No potential matches found"</p> }.into_any(),
                            Some(results) => view! { <ul class="signed-in-panel-list signed-in-search-results">{results.into_iter().map(|profile| {
                                let id = profile.user_id;
                                view! { <li class="signed-in-search-result">
                                    <A href=format!("/profile/{id}") attr:class="signed-in-search-profile">
                                        <span class="signed-in-panel-avatar">{profile.display_name.chars().next().unwrap_or('S').to_uppercase().to_string()}</span>
                                        <span><strong>{profile.display_name}</strong><small>{profile.cohort.unwrap_or_else(|| "NEU student".to_string())}</small></span>
                                    </A>
                                    {(!profile.bio.is_empty()).then(|| view! { <p class="signed-in-search-result-description">{profile.bio}</p> })}
                                    {(!profile.interests.is_empty()).then(|| view! { <p class="signed-in-search-result-interests"><strong>"Interests: "</strong>{profile.interests}</p> })}
                                    <button type="button" class="signed-in-send-request" on:click=move |_| create_request(id)>"Send request"</button>
                                </li> }
                            }).collect_view()}</ul> }.into_any()
                        }}
                        </Show>
                        <Show when=move || !explore_active.get() fallback=|| ()>
                        {move || panel_error.get().map(|error| view! { <p role="alert">{error}</p> })}
                        {move || match requests.get() {
                            None => view! { <p role="status">"Loading requests…"</p> }.into_any(),
                            Some(items) if items.is_empty() => view! { <p class="signed-in-panel-empty">"No pending or declined requests"</p> }.into_any(),
                            Some(items) => view! { <ul class="signed-in-panel-list">{items.into_iter().map(|item| {
                                let id = item.id;
                                let sender_id = item.sender_id;
                                let name = display_name(sender_id);
                                let is_pending = item.status == "pending";
                                view! { <li class="signed-in-request-item">
                                    <A href=format!("/profile/{sender_id}") attr:class="signed-in-request-profile">
                                        <span class="signed-in-panel-avatar">{name.chars().next().unwrap_or('S').to_uppercase().to_string()}</span>
                                        <span><strong>{name}</strong><small>{item.created_at} " · " {if is_pending { "Pending" } else { "Declined" }}</small></span>
                                    </A>
                                    {if is_pending { view! { <div class="signed-in-request-actions">
                                        <button type="button" class="request-accept" on:click=move |_| respond_to_request(id, true)>"Accept"</button>
                                        <button type="button" class="request-decline" on:click=move |_| respond_to_request(id, false)>"Decline"</button>
                                    </div> }.into_any() } else { view! { <span class="request-declined-label">"Declined"</span> }.into_any() }}
                                </li> }
                            }).collect_view()}</ul> }.into_any()
                        }}
                        </Show>
                    </Show>
                </aside>
            </Show>

            <div class="signed-in-content">{children()}</div>
        </div>
    }
}
