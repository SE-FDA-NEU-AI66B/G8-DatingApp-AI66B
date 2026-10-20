use crate::share::community::{CreateThread, CreateThreadComment, ThreadCard, ThreadComment};
use crate::share::register::MessageBody;
use leptos::prelude::*;
use leptos::tachys::html::event::SubmitEvent;
use serde::Serialize;

async fn response_error(response: gloo_net::http::Response, fallback: &str) -> String {
    response
        .json::<MessageBody>()
        .await
        .map(|body| body.message)
        .unwrap_or_else(|_| fallback.to_string())
}

async fn load_threads() -> Result<Vec<ThreadCard>, String> {
    use gloo_net::http::Request;

    let response = Request::get("/api/threads")
        .send()
        .await
        .map_err(|_| "Unable to reach the community feed.".to_string())?;
    if response.ok() {
        response
            .json::<Vec<ThreadCard>>()
            .await
            .map_err(|_| "The community feed returned an unexpected response.".to_string())
    } else {
        Err(response_error(response, "Unable to load the community feed.").await)
    }
}

async fn post_json<T: Serialize>(
    url: String,
    payload: &T,
) -> Result<gloo_net::http::Response, String> {
    use gloo_net::http::Request;

    let request = Request::post(&url)
        .json(payload)
        .map_err(|_| "Could not prepare your post.".to_string())?;
    request
        .send()
        .await
        .map_err(|_| "Unable to reach the community service.".to_string())
}

async fn load_comments(thread_id: i64) -> Result<Vec<ThreadComment>, String> {
    use gloo_net::http::Request;

    let response = Request::get(&format!("/api/threads/{thread_id}/comments"))
        .send()
        .await
        .map_err(|_| "Unable to load comments.".to_string())?;
    if response.ok() {
        response
            .json::<Vec<ThreadComment>>()
            .await
            .map_err(|_| "The comments service returned an unexpected response.".to_string())
    } else {
        Err(response_error(response, "Unable to load comments.").await)
    }
}

#[component]
fn ThreadPost(thread: ThreadCard) -> impl IntoView {
    let thread_id = thread.id;
    let thread_title = thread.title.clone();
    let (comment_count, set_comment_count) = signal(thread.comments_count);
    let (comments, set_comments) = signal(Vec::<ThreadComment>::new());
    let (comment, set_comment) = signal(String::new());
    let (error, set_error) = signal(None::<String>);
    let (busy, set_busy) = signal(false);

    Effect::new(move |_| {
        leptos::task::spawn_local(async move {
            match load_comments(thread_id).await {
                Ok(rows) => set_comments.set(rows),
                Err(message) => set_error.set(Some(message)),
            }
        });
    });

    let on_comment = move |event: SubmitEvent| {
        event.prevent_default();
        let body = comment.get_untracked().trim().to_string();
        if body.is_empty() || busy.get_untracked() {
            return;
        }
        set_busy.set(true);
        set_error.set(None);
        leptos::task::spawn_local(async move {
            match post_json(
                format!("/api/threads/{thread_id}/comments"),
                &CreateThreadComment { body },
            )
            .await
            {
                Ok(response) if response.ok() => match response.json::<ThreadComment>().await {
                    Ok(created) => {
                        set_comments.update(|rows| rows.push(created));
                        set_comment_count.update(|count| *count += 1);
                        set_comment.set(String::new());
                    }
                    Err(_) => set_error.set(Some(
                        "The comment service returned an unexpected response.".to_string(),
                    )),
                },
                Ok(response) => set_error.set(Some(
                    response_error(response, "Unable to post your comment.").await,
                )),
                Err(message) => set_error.set(Some(message)),
            }
            set_busy.set(false);
        });
    };

    view! {
        <article class="community-thread" aria-label={format!("Thread: {}", thread_title)}>
            <header class="community-thread-header">
                <div><h3 class="community-thread-title">{thread.title}</h3>
                    <p class="community-thread-meta">"Posted by "{thread.author_name.clone()}" · "{thread.created_at.clone()}</p></div>
                <span class="community-comment-count">{move || comment_count.get()}" comments"</span>
            </header>
            <p class="community-thread-body">{thread.body}</p>
            <section class="community-comments" aria-label="Comments">
                <h4 class="community-comments-heading">"Conversation"</h4>
                <ul class="community-comment-list">
                    <For each=move || comments.get() key=|item| item.id children=move |item| view! {
                        <li class="community-comment">
                            <strong>{item.author_name}</strong>
                            <span class="community-comment-meta">{item.created_at}</span>
                            <p>{item.body}</p>
                        </li>
                    } />
                </ul>
                <p class="community-error" role="alert">{move || error.get()}</p>
                <form class="community-comment-form" on:submit=on_comment>
                    <label class="community-sr-only" for={format!("community-comment-{thread_id}")}>"Write a comment"</label>
                    <input id={format!("community-comment-{thread_id}")} maxlength="2000" placeholder="Add to the conversation…"
                        bind:value=(comment, set_comment) required />
                    <button type="submit" disabled=move || busy.get() || comment.get().trim().is_empty()>
                        {move || if busy.get() { "Posting…" } else { "Comment" }}
                    </button>
                </form>
            </section>
        </article>
    }
}

#[component]
pub fn CommunityFeed() -> impl IntoView {
    let (threads, set_threads) = signal(Vec::<ThreadCard>::new());
    let (filter_text, set_filter_text) = signal(String::new());
    let (commented_only, set_commented_only) = signal(false);
    let (title, set_title) = signal(String::new());
    let (body, set_body) = signal(String::new());
    let (error, set_error) = signal(None::<String>);
    let (success, set_success) = signal(false);
    let (loading, set_loading) = signal(true);
    let (busy, set_busy) = signal(false);

    Effect::new(move |_| {
        leptos::task::spawn_local(async move {
            match load_threads().await {
                Ok(rows) => set_threads.set(rows),
                Err(message) => set_error.set(Some(message)),
            }
            set_loading.set(false);
        });
    });

    let on_create = move |event: SubmitEvent| {
        event.prevent_default();
        set_error.set(None);
        set_success.set(false);
        let post_body = body.get_untracked().trim().to_string();
        let typed_title = title.get_untracked().trim().to_string();
        let post_title = if typed_title.is_empty() {
            post_body
                .lines()
                .next()
                .unwrap_or("New thread")
                .chars()
                .take(120)
                .collect()
        } else {
            typed_title
        };
        let payload = CreateThread {
            title: post_title,
            body: post_body,
        };
        if payload.body.is_empty() || busy.get_untracked() {
            return;
        }
        set_busy.set(true);
        leptos::task::spawn_local(async move {
            match post_json("/api/threads".to_string(), &payload).await {
                Ok(response) if response.ok() => match response.json::<ThreadCard>().await {
                    Ok(created) => {
                        set_threads.update(|rows| rows.insert(0, created));
                        set_title.set(String::new());
                        set_body.set(String::new());
                        set_success.set(true);
                    }
                    Err(_) => set_error.set(Some(
                        "The thread service returned an unexpected response.".to_string(),
                    )),
                },
                Ok(response) => set_error.set(Some(
                    response_error(response, "Unable to create your thread.").await,
                )),
                Err(message) => set_error.set(Some(message)),
            }
            set_busy.set(false);
        });
    };

    let visible_threads = move || {
        let query = filter_text.get().trim().to_lowercase();
        let commented_only = commented_only.get();
        threads
            .get()
            .into_iter()
            .filter(|thread| {
                (!commented_only || thread.comments_count > 0)
                    && (query.is_empty()
                        || thread.title.to_lowercase().contains(&query)
                        || thread.body.to_lowercase().contains(&query)
                        || thread.author_name.to_lowercase().contains(&query))
            })
            .collect::<Vec<_>>()
    };

    view! {
        <section class="community-feed" aria-labelledby="community-feed-title">
            <header class="community-feed-heading">
                <p class="community-kicker">"CAMPUS CONVERSATIONS"</p>
                <h2 id="community-feed-title">"Community feed"</h2>
                <p>"Share what’s on your mind and join the conversation."</p>
            </header>
            <form class="community-create-form" on:submit=on_create>
                <h3>"Start a thread"</h3>
                <label for="community-thread-title">"Title (optional; generated from your post if blank)"</label>
                <input id="community-thread-title" maxlength="120" placeholder="What would you like to talk about?"
                    bind:value=(title, set_title) />
                <label for="community-thread-body">"Your post"</label>
                <textarea id="community-thread-body" maxlength="3000" rows="4" placeholder="Write a little more…"
                    bind:value=(body, set_body) required></textarea>
                <p class="community-error" role="alert">{move || error.get()}</p>
                <p class="community-success" role="status">{move || success.get().then_some("Your thread is live.")}</p>
                <button type="submit" disabled=move || busy.get() || body.get().trim().is_empty()>
                    {move || if busy.get() { "Starting thread…" } else { "Start a thread" }}
                </button>
            </form>
            <div class="community-filter" role="search">
                <label for="community-feed-search">"Filter threads"</label>
                <input id="community-feed-search" type="search" placeholder="Search titles, posts, or authors…"
                    bind:value=(filter_text, set_filter_text) />
                <label class="community-sr-only" for="community-comment-filter">"Thread activity"</label>
                <select id="community-comment-filter" on:change=move |event| {
                    set_commented_only.set(event_target_value(&event) == "commented");
                }>
                    <option value="all">"All threads"</option>
                    <option value="commented">"With comments"</option>
                </select>
            </div>
            <div class="community-thread-list" aria-live="polite">
                <Show when=move || loading.get()>
                    <p class="community-status" role="status">"Loading conversations…"</p>
                </Show>
                <Show when=move || !loading.get() && visible_threads().is_empty() && error.get().is_none()>
                    <p class="community-empty">{move || if threads.get().is_empty() {
                        "No threads yet. Start the first conversation."
                    } else {
                        "No threads match these filters."
                    }}</p>
                </Show>
                <For each=move || visible_threads() key=|thread| thread.id children=move |thread| view! {
                    <ThreadPost thread=thread />
                } />
            </div>
        </section>
    }
}
