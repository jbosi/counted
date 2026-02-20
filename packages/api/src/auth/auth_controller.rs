use dioxus::fullstack::Json;
use dioxus::prelude::*;
use shared::{Account, LoginPayload, RegisterPayload};

#[cfg(feature = "server")]
use crate::auth::auth_repository;
#[cfg(feature = "server")]
use crate::utils::get_current_account_id;
#[cfg(feature = "server")]
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
#[cfg(feature = "server")]
use axum::http::header::{HeaderValue, SET_COOKIE};
#[cfg(feature = "server")]
use chrono::{Duration, Utc};
#[cfg(feature = "server")]
use dioxus_fullstack::FullstackContext;
#[cfg(feature = "server")]
use uuid::Uuid;

#[post("/api/v1/auth/register")]
pub async fn register(Json(payload): Json<RegisterPayload>) -> Result<Account, ServerFnError> {
    if auth_repository::find_account_by_email(&payload.email).await?.is_some() {
        return Err(ServerFnError::new("Email already in use"));
    }

    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .to_string();

    let account_id =
        auth_repository::create_account(&payload.email, &password_hash, &payload.display_name)
            .await?;

    create_session_and_set_cookie(account_id).await?;

    let account = auth_repository::get_account_by_id(account_id)
        .await?
        .ok_or_else(|| ServerFnError::new("Account not found after creation"))?;

    Ok(account)
}

#[post("/api/v1/auth/login")]
pub async fn login(Json(payload): Json<LoginPayload>) -> Result<Account, ServerFnError> {
    let account_with_hash = auth_repository::find_account_by_email(&payload.email)
        .await?
        .ok_or_else(|| ServerFnError::new("Invalid email or password"))?;

    let parsed_hash = PasswordHash::new(&account_with_hash.password_hash)
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .map_err(|_| ServerFnError::new("Invalid email or password"))?;

    create_session_and_set_cookie(account_with_hash.id).await?;

    Ok(Account {
        id: account_with_hash.id,
        email: account_with_hash.email,
        display_name: account_with_hash.display_name,
        created_at: account_with_hash.created_at,
    })
}

#[post("/api/v1/auth/logout")]
pub async fn logout() -> Result<(), ServerFnError> {
    let ctx = FullstackContext::current()
        .ok_or_else(|| ServerFnError::new("No server context"))?;

    let session_id_opt = {
        let parts = ctx.parts_mut();
        parts
            .headers
            .get("cookie")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| {
                s.split(';')
                    .map(|p| p.trim())
                    .find(|p| p.starts_with("session_id="))
                    .and_then(|p| p.strip_prefix("session_id="))
                    .map(|p| p.to_owned())
            })
            .and_then(|id| id.parse::<Uuid>().ok())
    };

    if let Some(session_id) = session_id_opt {
        let _ = auth_repository::delete_session(session_id).await;
    }

    let clear_cookie =
        HeaderValue::from_static("session_id=; HttpOnly; SameSite=Lax; Path=/; Max-Age=0");
    ctx.add_response_header(SET_COOKIE, clear_cookie);

    Ok(())
}

#[get("/api/v1/auth/me")]
pub async fn me() -> Result<Option<Account>, ServerFnError> {
    let Some(account_id) = get_current_account_id().await else {
        return Ok(None);
    };
    Ok(auth_repository::get_account_by_id(account_id).await?)
}

/// Create a session in the DB and set the HttpOnly session cookie on the response.
#[cfg(feature = "server")]
async fn create_session_and_set_cookie(account_id: Uuid) -> Result<Uuid, ServerFnError> {
    let expires_at = (Utc::now() + Duration::days(30)).naive_utc();
    let session_id = auth_repository::create_session(account_id, expires_at).await?;

    let secure_attr =
        if std::env::var("COOKIE_SECURE").as_deref() == Ok("true") { "; Secure" } else { "" };

    let cookie_value = format!(
        "session_id={}; HttpOnly; SameSite=Lax; Path=/; Max-Age=2592000{}",
        session_id, secure_attr
    );

    let header_value =
        HeaderValue::from_str(&cookie_value).map_err(|e| ServerFnError::new(e.to_string()))?;

    let ctx = FullstackContext::current()
        .ok_or_else(|| ServerFnError::new("No server context"))?;
    ctx.add_response_header(SET_COOKIE, header_value);

    Ok(session_id)
}
