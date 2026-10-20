use crate::lib::share::register::{normalize_email, MessageBody, RegisterReq};
use actix_web::{post, web, HttpResponse};
use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2,
};
use rand::Rng;
use sqlx::PgPool;

#[post("/api/register")]
pub async fn register(pool: web::Data<PgPool>, body: web::Json<RegisterReq>) -> HttpResponse {
    let email = match normalize_email(&body.email) {
        Ok(email) => email,
        Err(message) => {
            return HttpResponse::BadRequest().json(MessageBody {
                message: message.into(),
            })
        }
    };
    let display_name = body.display_name.trim();
    if display_name.is_empty()
        || display_name.chars().count() > 80
        || display_name.chars().any(char::is_control)
    {
        return HttpResponse::BadRequest().json(MessageBody {
            message: "Enter a name using 1 to 80 characters.".into(),
        });
    }
    if body.password.chars().count() < 12 || body.password.len() > 1024 {
        return HttpResponse::BadRequest().json(MessageBody {
            message: "Password must contain at least 12 characters.".into(),
        });
    }

    let password = body.password.clone();
    let password_hash = match tokio::task::spawn_blocking(move || {
        let mut salt_bytes = [0_u8; 16];
        rand::rng().fill(&mut salt_bytes);
        let salt = SaltString::encode_b64(&salt_bytes).map_err(|error| error.to_string())?;
        Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|error| error.to_string())
    })
    .await
    {
        Ok(Ok(hash)) => hash,
        Ok(Err(error)) => {
            eprintln!("password hashing failed: {error}");
            return HttpResponse::InternalServerError().json(MessageBody {
                message: "Unable to create your account right now.".into(),
            });
        }
        Err(error) => {
            eprintln!("password hashing task failed: {error}");
            return HttpResponse::InternalServerError().json(MessageBody {
                message: "Unable to create your account right now.".into(),
            });
        }
    };

    match sqlx::query(
        "INSERT INTO users (email, display_name, password_hash)
         VALUES ($1, $2, $3)",
    )
    .bind(email)
    .bind(display_name)
    .bind(password_hash)
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Created().json(MessageBody {
            message: "Account created. Continue with Microsoft to verify your NEU Outlook address; then you can sign in with your email and password.".into(),
        }),
        Err(sqlx::Error::Database(error)) if error.is_unique_violation() => {
            HttpResponse::Conflict().json(MessageBody {
                message: "An account already exists for this email. Sign in with Microsoft or use password login if your email is verified.".into(),
            })
        }
        Err(error) => {
            eprintln!("account registration failed: {error}");
            HttpResponse::InternalServerError().json(MessageBody {
                message: "Unable to create your account right now.".into(),
            })
        }
    }
}
