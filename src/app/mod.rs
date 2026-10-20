use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{A, Route, Router, Routes},
    ParamSegment, StaticSegment, WildcardSegment,
};
mod admin;
mod api;
mod profile;
mod register;
mod requests;
mod study_date_form;
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
        <Title text="Study Date — meet between classes" />
        // content for this welcome page
        <Router>
            <header class="site-header">
                <A href="/" attr:class="brand" attr:aria-label="Study Date home">
                    <span class="brand-mark" attr:aria-hidden="true">"♡"</span>
                    <span>"studydate"</span>
                </A>
                <nav class="site-nav" attr:aria-label="Main navigation">
                    <A href="/feed">"Explore"</A>
                    <A href="/study-date/new">"Create a date"</A>
                    <A href="/requests">"Requests"</A>
                </nav>
                <div class="header-actions">
                    <A href="/login" attr:class="login-link">"Log in"</A>
                    <A href="/register" attr:class="button button-small">"Join free"</A>
                </div>
            </header>
            <main class="site-main">
                <Routes fallback=move || "Not found.">
                    <Route path=StaticSegment("login") view=login::App::App />
                    <Route path=StaticSegment("logout") view=login::App::Applogout />

                    <Route path=StaticSegment("register") view=register::RegisterPage />
                    <Route path=StaticSegment("feed") view=study_feed::StudyFeed />
                    <Route
                        path=(StaticSegment("study-date"), StaticSegment("new"))
                        view=study_date_form::StudyDateForm
                    />
                    <Route
                        path=(StaticSegment("profile"), ParamSegment("id"))
                        view=profile::ProfilePage
                    />
                    <Route path=StaticSegment("requests") view=requests::RequestsPage />
                    <Route path=StaticSegment("admin_console") view=admin::AdminDashboard />
                    <Route path=StaticSegment("") view=HomePage />
                    <Route path=WildcardSegment("any") view=NotFound />
                </Routes>
            </main>
        </Router>
    }
}
#[allow(non_snake_case)]
mod login;
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <section class="hero-section">
            <div class="hero-copy">
                <div class="eyebrow"><span class="eyebrow-dot"></span>"A little more than a study break"</div>
                <h1>"Meet someone "<span>"between classes."</span></h1>
                <p class="hero-description">
                    "Find your next favorite person over coffee, a study session, or a walk across campus. Made for the NEU community."
                </p>
                <div class="hero-actions">
                    <A href="/feed" attr:class="button button-primary">
                        "Find a study date "<span attr:aria-hidden="true">"↗"</span>
                    </A>
                    <A href="/register" attr:class="button button-quiet">"Create your account"</A>
                </div>
                <div class="social-proof">
                    <div class="avatar-stack" attr:aria-hidden="true">
                        <span class="mini-avatar avatar-coral">"A"</span>
                        <span class="mini-avatar avatar-lilac">"M"</span>
                        <span class="mini-avatar avatar-green">"L"</span>
                    </div>
                    <p><strong>"Your campus, your people."</strong><br />"Meet verified NEU students."</p>
                </div>
            </div>
            <div class="hero-art" attr:aria-label="Illustration of two students meeting for coffee">
                <div class="art-spark spark-one">"✦"</div>
                <div class="art-spark spark-two">"✳"</div>
                <div class="orbit orbit-one"></div>
                <div class="orbit orbit-two"></div>
                <div class="hero-note note-top"><span>"✦"</span>" A good kind of distraction"</div>
                <div class="meet-card">
                    <div class="meet-card-top">
                        <span class="live-pill"><i></i>"OPEN TO MEET"</span>
                        <span class="heart-chip">"♡"</span>
                    </div>
                    <div class="meet-illustration">
                        <div class="sun-disc"></div>
                        <div class="person person-left"><div class="person-hair"></div><div class="person-face"></div><div class="person-body"></div></div>
                        <div class="person person-right"><div class="person-hair"></div><div class="person-face"></div><div class="person-body"></div></div>
                        <div class="coffee-cup"><span></span></div>
                        <div class="table-line"></div>
                        <span class="tiny-heart">"♥"</span>
                    </div>
                    <div class="meet-card-bottom">
                        <div><span class="card-kicker">"TODAY · 2:30 PM"</span><h2>"Coffee & calculus?"</h2></div>
                        <span class="arrow-bubble">"↗"</span>
                    </div>
                </div>
                <div class="hero-note note-bottom"><span class="note-check">"✓"</span><span><strong>"NEU students only"</strong><br />"A familiar place to start"</span></div>
            </div>
        </section>

        <section class="how-section" attr:aria-labelledby="how-title">
            <div class="section-heading">
                <p class="section-kicker">"YOUR NEXT CHAPTER"</p>
                <h2 id="how-title">"A meet-cute, made simple."</h2>
                <p>"No swiping marathon. Just good people and an easy reason to say hello."</p>
            </div>
            <div class="feature-grid">
                <article class="feature-card feature-peach">
                    <div class="feature-icon">"✉"</div>
                    <span class="feature-number">"01"</span>
                    <h3>"Join your campus"</h3>
                    <p>"Sign up with your NEU email. A shared campus makes meeting feel a little more natural."</p>
                </article>
                <article class="feature-card feature-lilac">
                    <div class="feature-icon">"☕"</div>
                    <span class="feature-number">"02"</span>
                    <h3>"Pick a plan"</h3>
                    <p>"Post a time for coffee or a study session, then find someone whose schedule fits."</p>
                </article>
                <article class="feature-card feature-mint">
                    <div class="feature-icon">"♡"</div>
                    <span class="feature-number">"03"</span>
                    <h3>"See where it goes"</h3>
                    <p>"Send a match request and turn a shared study break into a real connection."</p>
                </article>
            </div>
        </section>

        <section class="closing-banner">
            <div class="closing-flower" attr:aria-hidden="true">"✿"</div>
            <div><p class="section-kicker">"GOOD THINGS START SMALL"</p><h2>"Your next favorite hello is out there."</h2></div>
            <A href="/register" attr:class="button button-light">"Get started "<span attr:aria-hidden="true">"↗"</span></A>
        </section>
        <footer class="site-footer"><span>"studydate "<span class="footer-heart">"♥"</span>" made for NEU"</span><span>"Meet kindly. Keep it real."</span></footer>
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
