use super::api;
use crate::share::{
    auth::SessionInfo,
    chat::{ChatMessage, ConversationCard, SendMessage},
};
use leptos::prelude::*;
use leptos_router::{components::A, hooks::use_params_map};

#[component]
pub fn ChatInbox() -> impl IntoView {
    let (conversations, set_conversations) = signal(None::<Vec<ConversationCard>>);
    let (error, set_error) = signal(None::<String>);
    Effect::new(move |_| {
        leptos::task::spawn_local(async move {
            match api::get_json::<Vec<ConversationCard>>("/api/conversations").await {
                Ok(items) => set_conversations.set(Some(items)),
                Err(e) => set_error.set(Some(e)),
            }
        })
    });
    view! {
        <section class="chat-page">
            <div class="chat-heading"><div><p class="section-kicker">"YOUR MATCHES"</p><h1>"Chat"</h1></div><p>"Every accepted match appears here, even before the first message."</p></div>
            {move || error.get().map(|e| view! { <p role="alert">{e}</p> })}
            {move || match conversations.get() {
                None => view! { <p role="status">"Loading conversations…"</p> }.into_any(),
                Some(items) if items.is_empty() => view! { <div class="chat-empty"><span>"♡"</span><h2>"No matches yet"</h2><p>"Accepted match requests will appear here. You can start chatting with any match."</p><A href="/explore" attr:class="button button-primary">"Explore students"</A></div> }.into_any(),
                Some(items) => view! { <ul class="conversation-list">{items.into_iter().map(|item| view! {
                    <li><A href=format!("/chat/{}", item.conversation_id) attr:class="conversation-link"><span class="conversation-avatar">{item.display_name.chars().next().unwrap_or('S').to_uppercase().to_string()}</span><span class="conversation-copy"><strong>{item.display_name}</strong><span>{item.last_message.unwrap_or_else(|| "Start the conversation".into())}</span></span><span class="conversation-arrow">"↗"</span></A></li>
                }).collect_view()}</ul> }.into_any()
            }}
        </section>
    }
}

#[component]
pub fn ChatRoom() -> impl IntoView {
    let params = use_params_map();
    let conversation_id = move || {
        params
            .get()
            .get("id")
            .and_then(|value| value.parse::<i64>().ok())
    };
    let (messages, set_messages) = signal(Vec::<ChatMessage>::new());
    let (body, set_body) = signal(String::new());
    let (me, set_me) = signal(None::<String>);
    let (error, set_error) = signal(None::<String>);
    let load = move |id: i64| {
        leptos::task::spawn_local(async move {
            if let Ok(items) =
                api::get_json::<Vec<ChatMessage>>(&format!("/api/conversations/{id}/messages"))
                    .await
            {
                set_messages.set(items);
            }
        });
    };
    Effect::new(move |_| {
        if let Some(id) = conversation_id() {
            load(id);
        }
        leptos::task::spawn_local(async move {
            if let Ok(session) = api::get_json::<SessionInfo>("/api/auth/session").await {
                set_me.set(Some(session.username));
            }
        });
    });
    let send = move |_| {
        let text = body.get_untracked();
        let Some(id) = conversation_id() else {
            return;
        };
        if text.trim().is_empty() {
            return;
        }
        set_error.set(None);
        leptos::task::spawn_local(async move {
            match api::post_json::<_, ChatMessage>(
                &format!("/api/conversations/{id}/messages"),
                &SendMessage { body: text },
            )
            .await
            {
                Ok(message) => {
                    set_messages.update(|list| list.push(message));
                    set_body.set(String::new());
                }
                Err(message) => set_error.set(Some(message)),
            }
        });
    };
    view! {
        <section class="chat-page chat-room">
            <div class="chat-room-heading"><A href="/chat" attr:class="chat-back">"← All messages"</A><h1>"Conversation"</h1><button class="chat-refresh" on:click=move |_| if let Some(id) = conversation_id() { load(id); } aria-label="Refresh messages">"↻"</button></div>
            {move || error.get().map(|e| view! { <p role="alert">{e}</p> })}
            <div class="chat-messages" aria-live="polite">
                {move || messages.get().into_iter().map(|message| {
                    let mine = me.get().is_some_and(|name| name == message.sender_name);
                    view! { <article class=if mine { "chat-bubble chat-bubble-mine" } else { "chat-bubble" }><p>{message.body}</p><small>{message.sender_name} " · " {message.created_at}</small></article> }
                }).collect_view()}
            </div>
            <form class="chat-compose" on:submit=move |ev| { ev.prevent_default(); send(()); }>
                <input type="text" maxlength="2000" placeholder="Write a message…" prop:value=move || body.get() on:input=move |ev| set_body.set(event_target_value(&ev)) aria-label="Message" />
                <button type="submit">"Send"</button>
            </form>
        </section>
    }
}
