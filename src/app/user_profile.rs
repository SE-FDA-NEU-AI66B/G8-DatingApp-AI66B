use super::api;
use crate::share::user_profile::UserProfile;
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn UserProfilePage() -> impl IntoView {
    let (display_name, set_display_name) = signal(String::new());
    let (bio, set_bio) = signal(String::new());
    let (cohort, set_cohort) = signal(String::new());
    let (interests, set_interests) = signal(String::new());
    let (loading, set_loading) = signal(true);
    let (saving, set_saving) = signal(false);
    let (error, set_error) = signal(None::<String>);
    let (success, set_success) = signal(false);

    Effect::new(move |_| {
        leptos::task::spawn_local(async move {
            match api::get_json::<UserProfile>("/api/user-profile").await {
                Ok(profile) => {
                    set_display_name.set(profile.display_name);
                    set_bio.set(profile.bio);
                    set_cohort.set(profile.cohort.unwrap_or_default());
                    set_interests.set(profile.interests);
                }
                Err(message) => set_error.set(Some(message)),
            }
            set_loading.set(false);
        });
    });

    view! {
        <section class="user-profile-editor" aria-labelledby="user-profile-title">
            <A href="/userhome" attr:class="profile-editor-back">"← Your home"</A>
            <div class="profile-editor-heading">
                <p class="section-kicker">"MAKE IT YOURS"</p>
                <h1 id="user-profile-title">"Your profile"</h1>
                <p>"Add a little about yourself so your future matches have an easy place to start."</p>
            </div>
            {move || if loading.get() { view! { <p role="status">"Loading your profile…"</p> }.into_any() } else { view! {
                <form class="profile-editor-form" on:submit=move |ev| {
                    ev.prevent_default();
                    set_saving.set(true); set_error.set(None); set_success.set(false);
                    let profile = UserProfile {
                        display_name: display_name.get_untracked(),
                        bio: bio.get_untracked(),
                        cohort: Some(cohort.get_untracked()),
                        interests: interests.get_untracked(),
                    };
                    leptos::task::spawn_local(async move {
                        match api::put_json::<_, UserProfile>("/api/user-profile", &profile).await {
                            Ok(saved) => {
                                set_display_name.set(saved.display_name);
                                set_bio.set(saved.bio);
                                set_cohort.set(saved.cohort.unwrap_or_default());
                                set_interests.set(saved.interests);
                                set_success.set(true);
                            }
                            Err(message) => set_error.set(Some(message)),
                        }
                        set_saving.set(false);
                    });
                }>
                    <label for="profile-display-name">"Display name"</label>
                    <input id="profile-display-name" type="text" maxlength="60" required prop:value=move || display_name.get() on:input=move |ev| set_display_name.set(event_target_value(&ev)) />
                    <label for="profile-cohort">"Cohort (optional)"</label>
                    <input id="profile-cohort" type="text" maxlength="24" placeholder="e.g. K68" prop:value=move || cohort.get() on:input=move |ev| set_cohort.set(event_target_value(&ev)) />
                    <label for="profile-bio">"About me"</label>
                    <textarea id="profile-bio" maxlength="500" rows="5" placeholder="What do you enjoy? What would you like to meet up for?" prop:value=move || bio.get() on:input=move |ev| set_bio.set(event_target_value(&ev))></textarea>
                    <label for="profile-interests">"Interests"</label>
                    <textarea id="profile-interests" maxlength="300" rows="3" placeholder="Coffee, films, hiking…" prop:value=move || interests.get() on:input=move |ev| set_interests.set(event_target_value(&ev))></textarea>
                    {move || error.get().map(|message| view! { <p role="alert">{message}</p> })}
                    {move || success.get().then(|| view! { <p role="status">"Your profile has been saved."</p> })}
                    <button type="submit" disabled=move || saving.get()>{move || if saving.get() { "Saving…" } else { "Save profile" }}</button>
                </form>
            }.into_any() }}
        </section>
    }
}
