use crate::lib::share::{
    auth::{AuthError, LoginRequest},
    register::normalize_email,
};
use actix_web::{
    cookie::{Cookie, SameSite},
    web, HttpRequest, HttpResponse,
};
use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
};
use openidconnect::{
    core::{CoreClient, CoreProviderMetadata},
    reqwest, AuthorizationCode, ClientId, ClientSecret, CsrfToken, IssuerUrl, Nonce,
    PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope, TokenResponse,
};
use rand::Rng;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

const SESSION_COOKIE: &str = "datingapp_session";
const OAUTH_STATE_COOKIE: &str = "oauth_state";
const OAUTH_NONCE_COOKIE: &str = "oauth_nonce";
const OAUTH_VERIFIER_COOKIE: &str = "oauth_verifier";

#[derive(Debug, Clone, FromRow)]
pub struct AuthenticatedUser {
    pub role: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, FromRow)]
pub struct CurrentUser {
    pub id: Uuid,
    pub role: String,
}

#[derive(Debug, FromRow)]
struct PasswordLoginUser {
    id: Uuid,
    password_hash: Option<String>,
    email_verified: bool,
}

fn auth_error(status: actix_web::http::StatusCode, message: &str) -> HttpResponse {
    HttpResponse::build(status).json(AuthError {
        message: message.to_string(),
    })
}

fn hash_token(token: &str) -> String {
    hex::encode(Sha256::digest(token.as_bytes()))
}

fn new_token() -> String {
    let mut bytes = [0_u8; 32];
    rand::rng().fill(&mut bytes);
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn session_cookie(token: String) -> Cookie<'static> {
    Cookie::build(SESSION_COOKIE, token)
        .http_only(true)
        .secure(!cfg!(debug_assertions))
        .same_site(SameSite::Lax)
        .path("/")
        .max_age(actix_web::cookie::time::Duration::hours(8))
        .finish()
}

async fn create_session(pool: &PgPool, user_id: Uuid) -> Result<String, sqlx::Error> {
    let token = new_token();
    sqlx::query(
        "INSERT INTO auth_cookies (token_hash, user_id, expires_at)
         VALUES ($1, $2, now() + interval '8 hours')",
    )
    .bind(hash_token(&token))
    .bind(user_id)
    .execute(pool)
    .await?;
    Ok(token)
}

pub async fn password_login(
    pool: web::Data<PgPool>,
    body: web::Json<LoginRequest>,
) -> HttpResponse {
    let email = match normalize_email(&body.email) {
        Ok(email) => email,
        Err(_) => {
            return auth_error(
                actix_web::http::StatusCode::UNAUTHORIZED,
                "Invalid email or password.",
            )
        }
    };
    let user = match sqlx::query_as::<_, PasswordLoginUser>(
        "SELECT id, password_hash, email_verified FROM users WHERE email = $1",
    )
    .bind(email)
    .fetch_optional(pool.get_ref())
    .await
    {
        Ok(Some(user)) => user,
        Ok(None) => {
            return auth_error(
                actix_web::http::StatusCode::UNAUTHORIZED,
                "Invalid email or password.",
            )
        }
        Err(error) => {
            eprintln!("password login lookup failed: {error}");
            return auth_error(
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Authentication service is temporarily unavailable.",
            );
        }
    };
    let Some(password_hash) = user.password_hash else {
        return auth_error(
            actix_web::http::StatusCode::UNAUTHORIZED,
            "Invalid email or password.",
        );
    };
    let password = body.password.clone();
    let verified = tokio::task::spawn_blocking(move || {
        PasswordHash::new(&password_hash).ok().is_some_and(|hash| {
            Argon2::default()
                .verify_password(password.as_bytes(), &hash)
                .is_ok()
        })
    })
    .await
    .unwrap_or(false);
    if !verified || !user.email_verified {
        return auth_error(
            actix_web::http::StatusCode::UNAUTHORIZED,
            "Invalid email or password.",
        );
    }
    match create_session(pool.get_ref(), user.id).await {
        Ok(token) => HttpResponse::Ok().cookie(session_cookie(token)).finish(),
        Err(error) => {
            eprintln!("password login session creation failed: {error}");
            auth_error(
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Authentication service is temporarily unavailable.",
            )
        }
    }
}

/// Begin Microsoft's OAuth2/OIDC authorization-code flow with PKCE.
pub async fn login() -> HttpResponse {
    let result = async {
        let tenant = std::env::var("MICROSOFT_TENANT_ID")
            .map_err(|_| "MICROSOFT_TENANT_ID is not configured")?;
        let client_id =
            std::env::var("MICROSOFT_CLIENT_ID").map_err(|_| "OAuth2 is not configured")?;
        let redirect_uri =
            std::env::var("MICROSOFT_REDIRECT_URI").map_err(|_| "OAuth2 is not configured")?;
        let tenant =
            Uuid::parse_str(&tenant).map_err(|_| "MICROSOFT_TENANT_ID must be a tenant GUID")?;
        let issuer = IssuerUrl::new(format!("https://login.microsoftonline.com/{tenant}/v2.0"))
            .map_err(|_| "Invalid Microsoft tenant configuration")?;
        let http_client = reqwest::ClientBuilder::new()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .map_err(|_| "OAuth2 client initialization failed")?;
        let metadata = CoreProviderMetadata::discover_async(issuer, &http_client)
            .await
            .map_err(|_| "Could not load Microsoft sign-in configuration")?;
        let client = CoreClient::from_provider_metadata(metadata, ClientId::new(client_id), None)
            .set_redirect_uri(
                RedirectUrl::new(redirect_uri).map_err(|_| "Invalid OAuth callback URL")?,
            );
        let (challenge, verifier) = PkceCodeChallenge::new_random_sha256();
        let (url, state, nonce) = client
            .authorize_url(
                openidconnect::core::CoreAuthenticationFlow::AuthorizationCode,
                CsrfToken::new_random,
                Nonce::new_random,
            )
            .add_scope(Scope::new("openid".into()))
            .add_scope(Scope::new("profile".into()))
            .add_scope(Scope::new("email".into()))
            .set_pkce_challenge(challenge)
            .url();
        Ok::<_, &'static str>((url, state, nonce, verifier))
    }
    .await;

    match result {
        Ok((url, state, nonce, verifier)) => {
            let secure = !cfg!(debug_assertions);
            let lifetime = actix_web::cookie::time::Duration::minutes(10);
            HttpResponse::Found()
                .append_header((actix_web::http::header::LOCATION, url.to_string()))
                .cookie(
                    Cookie::build(OAUTH_STATE_COOKIE, state.secret().clone())
                        .http_only(true)
                        .secure(secure)
                        .same_site(SameSite::Lax)
                        .path("/api/auth")
                        .max_age(lifetime)
                        .finish(),
                )
                .cookie(
                    Cookie::build(OAUTH_NONCE_COOKIE, nonce.secret().clone())
                        .http_only(true)
                        .secure(secure)
                        .same_site(SameSite::Lax)
                        .path("/api/auth")
                        .max_age(lifetime)
                        .finish(),
                )
                .cookie(
                    Cookie::build(OAUTH_VERIFIER_COOKIE, verifier.secret().clone())
                        .http_only(true)
                        .secure(secure)
                        .same_site(SameSite::Lax)
                        .path("/api/auth")
                        .max_age(lifetime)
                        .finish(),
                )
                .finish()
        }
        Err(message) => HttpResponse::ServiceUnavailable().body(message),
    }
}

#[derive(Deserialize)]
pub struct OAuthCallback {
    code: Option<String>,
    state: Option<String>,
    error: Option<String>,
}

fn clear_oauth_cookies(response: &mut HttpResponse) {
    let secure = !cfg!(debug_assertions);
    for name in [
        OAUTH_STATE_COOKIE,
        OAUTH_NONCE_COOKIE,
        OAUTH_VERIFIER_COOKIE,
    ] {
        let cookie = Cookie::build(name, "")
            .http_only(true)
            .secure(secure)
            .same_site(SameSite::Lax)
            .path("/api/auth")
            .max_age(actix_web::cookie::time::Duration::ZERO)
            .finish();
        let _ = response.add_removal_cookie(&cookie);
    }
}

pub async fn callback(
    request: HttpRequest,
    query: web::Query<OAuthCallback>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    if query.error.is_some() {
        return HttpResponse::Unauthorized().body("Microsoft sign-in was cancelled or denied.");
    }
    let (
        Some(code),
        Some(returned_state),
        Some(state_cookie),
        Some(nonce_cookie),
        Some(verifier_cookie),
    ) = (
        query.code.as_deref(),
        query.state.as_deref(),
        request.cookie(OAUTH_STATE_COOKIE),
        request.cookie(OAUTH_NONCE_COOKIE),
        request.cookie(OAUTH_VERIFIER_COOKIE),
    )
    else {
        return HttpResponse::BadRequest().body("Invalid or expired Microsoft sign-in response.");
    };
    let expected_hash = Sha256::digest(state_cookie.value().as_bytes());
    let actual_hash = Sha256::digest(returned_state.as_bytes());
    if expected_hash != actual_hash {
        return HttpResponse::BadRequest().body("Invalid Microsoft sign-in state.");
    }

    let tenant = match std::env::var("MICROSOFT_TENANT_ID") {
        Ok(tenant) => tenant,
        Err(_) => {
            return HttpResponse::ServiceUnavailable().body("Microsoft tenant is not configured.")
        }
    };
    let (client_id, client_secret, redirect_uri) = match (
        std::env::var("MICROSOFT_CLIENT_ID"),
        std::env::var("MICROSOFT_CLIENT_SECRET"),
        std::env::var("MICROSOFT_REDIRECT_URI"),
    ) {
        (Ok(id), Ok(secret), Ok(uri)) => (id, secret, uri),
        _ => {
            return HttpResponse::ServiceUnavailable().body("Microsoft sign-in is not configured.")
        }
    };
    let tenant = match Uuid::parse_str(&tenant) {
        Ok(tenant) => tenant,
        Err(_) => {
            return HttpResponse::ServiceUnavailable().body("Microsoft tenant ID must be a GUID.")
        }
    };
    let issuer = match IssuerUrl::new(format!("https://login.microsoftonline.com/{tenant}/v2.0")) {
        Ok(issuer) => issuer,
        Err(_) => {
            return HttpResponse::ServiceUnavailable()
                .body("Invalid Microsoft tenant configuration.")
        }
    };
    let http_client = match reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .build()
    {
        Ok(client) => client,
        Err(_) => {
            return HttpResponse::ServiceUnavailable().body("Microsoft sign-in is unavailable.")
        }
    };
    let metadata = match CoreProviderMetadata::discover_async(issuer, &http_client).await {
        Ok(metadata) => metadata,
        Err(error) => {
            eprintln!("Microsoft discovery failed: {error}");
            return HttpResponse::ServiceUnavailable().body("Microsoft sign-in is unavailable.");
        }
    };
    let client = CoreClient::from_provider_metadata(
        metadata,
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
    )
    .set_redirect_uri(match RedirectUrl::new(redirect_uri) {
        Ok(uri) => uri,
        Err(_) => return HttpResponse::ServiceUnavailable().body("Invalid OAuth callback URL."),
    });
    let exchange = match client.exchange_code(AuthorizationCode::new(code.to_owned())) {
        Ok(exchange) => exchange,
        Err(error) => {
            eprintln!("OAuth code request creation failed: {error}");
            return HttpResponse::Unauthorized().body("Microsoft sign-in could not be completed.");
        }
    };
    let token_response = match exchange
        .set_pkce_verifier(PkceCodeVerifier::new(verifier_cookie.value().to_owned()))
        .request_async(&http_client)
        .await
    {
        Ok(tokens) => tokens,
        Err(error) => {
            eprintln!("OAuth code exchange failed: {error}");
            return HttpResponse::Unauthorized().body("Microsoft sign-in could not be completed.");
        }
    };
    let Some(id_token) = token_response.id_token() else {
        return HttpResponse::Unauthorized().body("Microsoft did not return an identity token.");
    };
    let nonce = Nonce::new(nonce_cookie.value().to_owned());
    let claims = match id_token.claims(&client.id_token_verifier(), &nonce) {
        Ok(claims) => claims,
        Err(error) => {
            eprintln!("Microsoft identity token validation failed: {error}");
            return HttpResponse::Unauthorized()
                .body("Microsoft identity token validation failed.");
        }
    };
    let Some(email_claim) = claims.email() else {
        return HttpResponse::Forbidden()
            .body("Your Microsoft account did not provide an email address.");
    };
    let email = match normalize_email(email_claim.as_str()) {
        Ok(email) => email,
        Err(_) => {
            return HttpResponse::Forbidden().body("Use your verified @neu.edu.vn Outlook account.")
        }
    };
    let is_admin = std::env::var("ADMIN_EMAIL")
        .ok()
        .is_some_and(|admin_email| admin_email.trim().eq_ignore_ascii_case(&email));
    let initial_name = email.split('@').next().unwrap_or_default().to_string();
    let user_id = match sqlx::query_scalar::<_, Uuid>(
        "INSERT INTO users (email, display_name, email_verified, role)
         VALUES ($1, $2, TRUE, CASE WHEN $3 THEN 'admin' ELSE 'user' END)
         ON CONFLICT (email) DO UPDATE SET
           email_verified = TRUE,
           role = CASE WHEN EXCLUDED.role = 'admin' THEN 'admin' ELSE users.role END,
           display_name = CASE WHEN users.display_name = ''
             THEN EXCLUDED.display_name ELSE users.display_name END
         RETURNING id",
    )
    .bind(email)
    .bind(initial_name)
    .bind(is_admin)
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(id) => id,
        Err(error) => {
            eprintln!("OAuth account provisioning failed: {error}");
            return HttpResponse::InternalServerError().body("Unable to create your account.");
        }
    };
    let token = match create_session(pool.get_ref(), user_id).await {
        Ok(token) => token,
        Err(error) => {
            eprintln!("OAuth session creation failed: {error}");
            return HttpResponse::InternalServerError().body("Unable to create your session.");
        }
    };
    let mut response = HttpResponse::Found();
    response.append_header((actix_web::http::header::LOCATION, "/"));
    let mut response = response.cookie(session_cookie(token)).finish();
    clear_oauth_cookies(&mut response);
    response
}

pub async fn logout(request: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    if let Some(cookie) = request.cookie(SESSION_COOKIE) {
        if let Err(error) = sqlx::query("DELETE FROM auth_cookies WHERE token_hash = $1")
            .bind(hash_token(cookie.value()))
            .execute(pool.get_ref())
            .await
        {
            eprintln!("logout failed: {error}");
            return auth_error(
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Unable to end the session.",
            );
        }
    }

    HttpResponse::NoContent()
        .cookie(
            Cookie::build(SESSION_COOKIE, "")
                .http_only(true)
                .secure(!cfg!(debug_assertions))
                .same_site(SameSite::Lax)
                .path("/")
                .max_age(actix_web::cookie::time::Duration::ZERO)
                .finish(),
        )
        .finish()
}

pub async fn require_admin(
    request: &HttpRequest,
    pool: &PgPool,
) -> Result<AuthenticatedUser, HttpResponse> {
    let Some(cookie) = request.cookie(SESSION_COOKIE) else {
        return Err(auth_error(
            actix_web::http::StatusCode::UNAUTHORIZED,
            "Authentication is required.",
        ));
    };

    let user = sqlx::query_as::<_, AuthenticatedUser>(
        "SELECT u.role
         FROM auth_cookies s
         JOIN users u ON u.id = s.user_id
         WHERE s.token_hash = $1
           AND s.expires_at > now()",
    )
    .bind(hash_token(cookie.value()))
    .fetch_optional(pool)
    .await
    .map_err(|error| {
        eprintln!("admin authorization lookup failed: {error}");
        auth_error(
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            "Authentication service is temporarily unavailable.",
        )
    })?;

    let Some(user) = user else {
        return Err(auth_error(
            actix_web::http::StatusCode::UNAUTHORIZED,
            "Authentication is required.",
        ));
    };

    if user.role != "admin" {
        return Err(auth_error(
            actix_web::http::StatusCode::FORBIDDEN,
            "Administrator access is required.",
        ));
    }

    Ok(user)
}

pub async fn require_user(
    request: &HttpRequest,
    pool: &PgPool,
) -> Result<CurrentUser, HttpResponse> {
    let Some(cookie) = request.cookie(SESSION_COOKIE) else {
        return Err(auth_error(
            actix_web::http::StatusCode::UNAUTHORIZED,
            "Authentication is required.",
        ));
    };
    let user = sqlx::query_as::<_, CurrentUser>(
        "SELECT u.id, u.role
         FROM auth_cookies s
         JOIN users u ON u.id = s.user_id
         WHERE s.token_hash = $1 AND s.expires_at > now()",
    )
    .bind(hash_token(cookie.value()))
    .fetch_optional(pool)
    .await
    .map_err(|error| {
        eprintln!("user authorization lookup failed: {error}");
        auth_error(
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            "Authentication service is temporarily unavailable.",
        )
    })?;
    user.ok_or_else(|| {
        auth_error(
            actix_web::http::StatusCode::UNAUTHORIZED,
            "Authentication is required.",
        )
    })
}

pub async fn configure_admin(pool: &PgPool, email: Option<&str>) -> Result<(), sqlx::Error> {
    let Some(email) = email else {
        return Ok(());
    };

    sqlx::query("UPDATE users SET role = 'admin' WHERE email = $1")
        .bind(email.trim().to_lowercase())
        .execute(pool)
        .await?;
    Ok(())
}
