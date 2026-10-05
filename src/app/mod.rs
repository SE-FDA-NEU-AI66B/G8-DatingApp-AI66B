use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    ParamSegment, StaticSegment, WildcardSegment,
};
mod admin;
mod api;
mod profile;
mod requests;
mod study_feed;
#[allow(non_snake_case)]
#[component]
pub fn app() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/datingapp.css" />
        // sets the document title
        <Title text="Welcome to Leptos" />
        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=move || "Not found.">
                    <Route path=StaticSegment("") view=HomePage />
                    <Route path=StaticSegment("register") view=register::RegisterPage />
                    <Route path=StaticSegment("feed") view=study_feed::StudyFeed />
                    <Route path=(StaticSegment("profile"), ParamSegment("id")) view=profile::ProfilePage />
                    <Route path=StaticSegment("requests") view=requests::RequestsPage />
                    <Route path=StaticSegment("admin_console") view=admin::AdminDashboard />
                    <Route path=WildcardSegment("any") view=NotFound />
                </Routes>
            </main>
        </Router>
    }
}

#[allow(non_snake_case)]
mod p15;
#[allow(non_snake_case)]
mod p3;
#[allow(non_snake_case)]
mod p4;
#[allow(non_snake_case)]
#[path = "register.ss"]
mod register;
#[component]
fn HomePage() -> impl IntoView {
    view! {
        {p3::App()}
        {p4::App()}
        {p15::App()}
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }
    view! { <h1>"Not Found"</h1> }
}
