use super::api;
use crate::share::{chat::ConversationCard, matching::StudyDateCard};
use leptos::prelude::*;
use leptos_router::components::A;

#[derive(Clone, Copy, PartialEq, Eq)]
enum SidePanel {
    Chat,
    Dates,
}

#[component]
pub fn SignedInLayout(children: ChildrenFn) -> impl IntoView {
    let (pinned, set_pinned) = signal(true);
    let (expanded, set_expanded) = signal(None::<SidePanel>);
    let (conversations, set_conversations) = signal(None::<Vec<ConversationCard>>);
    let (dates, set_dates) = signal(None::<Vec<StudyDateCard>>);
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

    view! {
        <div class=move || if expanded.get().is_some() { "signed-in-layout has-panel" } else { "signed-in-layout" }>
            <aside class=move || if pinned.get() { "signed-in-sidebar is-pinned" } else { "signed-in-sidebar" }
                aria-label="Signed-in navigation">
                <div class="signed-in-sidebar-heading">
                    <button type="button" class="sidebar-pin" aria-label=move || if pinned.get() { "Unpin navigation" } else { "Pin navigation" }
                        aria-pressed=move || pinned.get()
                        on:click=move |_| set_pinned.update(|value| *value = !*value)>
                        {move || if pinned.get() { "📌" } else { "📍" }}
                    </button>
                </div>
                <nav class="signed-in-nav" aria-label="Workspace">
                    <A href="/userhome" attr:class="signed-in-nav-link" attr:title="Home" attr:aria-label="Home">
                        <span aria-hidden="true">"⌂"</span><span class="signed-in-nav-label">"Home"</span>
                    </A>
                    <button type="button" class="signed-in-nav-link" title="Chat" aria-label="Chat"
                        aria-expanded=move || expanded.get() == Some(SidePanel::Chat) on:click=open_chat>
                        <span aria-hidden="true">"✉"</span>
                    </button>
                    <button type="button" class="signed-in-nav-link" title="Study date" aria-label="Study date"
                        aria-expanded=move || expanded.get() == Some(SidePanel::Dates) on:click=open_dates>
                        <span aria-hidden="true">"＋"</span>
                    </button>
                    <A href="/requests" attr:class="signed-in-nav-link" attr:title="Requests" attr:aria-label="Requests">
                        <span aria-hidden="true">"♡"</span><span class="signed-in-nav-label">"Requests"</span>
                    </A>
                </nav>
            </aside>

            <Show when=move || expanded.get().is_some()>
                <aside class="signed-in-panel" aria-label=move || match expanded.get() {
                    Some(SidePanel::Chat) => "Chat list",
                    Some(SidePanel::Dates) => "Study dates",
                    None => "Workspace panel",
                }>
                    <Show when=move || expanded.get() == Some(SidePanel::Chat) fallback=|| ()>
                        <div class="signed-in-panel-heading"><h2>"Chat"</h2></div>
                        {move || panel_error.get().map(|error| view! { <p role="alert">{error}</p> })}
                        {move || match conversations.get() {
                            None => view! { <p role="status">"Loading chats…"</p> }.into_any(),
                            Some(items) if items.is_empty() => view! { <p class="signed-in-panel-empty">"No chats yet"</p> }.into_any(),
                            Some(items) => view! { <ul class="signed-in-panel-list">{items.into_iter().map(|item| view! {
                                <li><A href=format!("/chat/{}", item.conversation_id) attr:class="signed-in-panel-link">
                                    <span class="signed-in-panel-avatar">{item.display_name.chars().next().unwrap_or('S').to_uppercase().to_string()}</span>
                                    <span class="signed-in-panel-copy"><strong>{item.display_name}</strong><small>{item.last_message.unwrap_or_else(|| "Start the conversation".into())}</small></span>
                                </A></li>
                            }).collect_view()}</ul> }.into_any()
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
                            Some(items) => view! { <ul class="signed-in-panel-list">{items.into_iter().map(|item| view! {
                                <li><A href="/feed" attr:class="signed-in-panel-link signed-in-date-link">
                                    <strong>{item.starts_at}</strong><small>{if item.is_mine { "Your study date".to_string() } else { "Open study date feed".to_string() }}</small>
                                </A></li>
                            }).collect_view()}</ul> }.into_any()
                        }}
                    </Show>
                </aside>
            </Show>

            <div class="signed-in-content">{children()}</div>
        </div>
    }
}
