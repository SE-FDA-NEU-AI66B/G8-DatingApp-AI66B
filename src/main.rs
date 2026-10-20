mod ss;
use std::env;
mod worker;
pub use datingapp as lib;

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ = dotenvy::dotenv();
    use actix_files::Files;
    use actix_web::*;
    use leptos::config::get_configuration;
    use leptos::prelude::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use leptos_meta::MetaTags;
    use lib::app::*;
    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect(&ss::functions::get_db_uri())
        .await
        .expect("cannot connect to PostgreSQL (check DATABASE_URL and `mise run database`)");
    ss::auth::configure_admin(&pool, env::var("ADMIN_EMAIL").ok().as_deref())
        .await
        .expect("admin role configuration failed");
    let pool = web::Data::new(pool);

    let server = HttpServer::new(move || {
        // Generate the list of routes in your Leptos App
        let routes = generate_route_list(App);
        let leptos_options = &conf.leptos_options;
        let site_root = leptos_options.site_root.clone().to_string();
        println!("listening on http://{}", &addr);

        App::new()
            .configure(worker::config)
            .app_data(pool.clone())
            .service(
                actix_web::web::resource("/api/auth/login")
                    .route(actix_web::web::get().to(ss::auth::login)),
            )
            .service(
                actix_web::web::resource("/api/auth/password")
                    .route(actix_web::web::post().to(ss::auth::password_login)),
            )
            .service(
                actix_web::web::resource("/api/auth/session")
                    .route(actix_web::web::get().to(ss::auth::session)),
            )
            .service(
                actix_web::web::resource("/api/auth/callback")
                    .route(actix_web::web::get().to(ss::auth::callback)),
            )
            .service(
                actix_web::web::resource("/api/auth/logout")
                    .route(actix_web::web::post().to(ss::auth::logout)),
            )
            .service(ss::admin::dashboard_statistics)
            .service(ss::admin::health_status)
            .service(ss::register::register)
            .service(
                actix_web::web::resource("/api/study-dates")
                    .route(actix_web::web::get().to(ss::study_date::list_study_dates))
                    .route(actix_web::web::post().to(ss::study_date::create_study_date)),
            )
            .service(ss::matching::get_profile)
            .service(ss::matching::send_request)
            .service(ss::matching::incoming_requests)
            .service(ss::matching::matched_users)
            .service(ss::matching::accept_request)
            .service(ss::matching::decline_request)
            .service(ss::chat::conversations)
            .service(ss::chat::messages)
            .service(ss::chat::send_message)
            .service(ss::community::list_threads)
            .service(ss::community::create_thread)
            .service(ss::community::list_thread_comments)
            .service(ss::community::create_thread_comment)
            .service(ss::user_profile::get_user_profile)
            .service(ss::user_profile::save_user_profile)
            // serve JS/WASM/CSS from `pkg`
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            // serve other assets from the `assets` directory
            .service(Files::new("/assets", &site_root))
            // serve the favicon from /favicon.ico
            .service(favicon)
            .leptos_routes(routes, {
                let leptos_options = leptos_options.clone();
                move || {
                    view! {
                        <!DOCTYPE html>
                        <html lang="en">
                            <head>
                                <meta charset="utf-8" />
                                <meta
                                    name="viewport"
                                    content="width=device-width, initial-scale=1"
                                />
                                <AutoReload options=leptos_options.clone() />
                                <HydrationScripts options=leptos_options.clone() />
                                <MetaTags />
                            </head>
                            <body>
                                <App />
                            </body>
                        </html>
                    }
                }
            })
            .app_data(web::Data::new(leptos_options.to_owned()))
        //.wrap(middleware::Compress::default())
    });
    let tls_config = if !cfg!(debug_assertions) {
        Some(ss::functions::get_tls_config())
    } else {
        None
    };
    if tls_config.is_none() {
        server.bind(&addr)
    } else {
        server.bind_rustls_0_23(&addr, tls_config.unwrap())
    }?
    .run()
    .await
}

#[cfg(feature = "ssr")]
#[actix_web::get("favicon.ico")]
async fn favicon(
    leptos_options: actix_web::web::Data<leptos::config::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/favicon.ico"
    ))?)
}
