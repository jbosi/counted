use chrono::NaiveDateTime;
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

fn is_account_locked(locked_until: Option<NaiveDateTime>, now: NaiveDateTime) -> bool {
    locked_until.map_or(false, |l| l > now)
}

#[post("/api/v1/auth/register")]
pub async fn register(Json(payload): Json<RegisterPayload>) -> Result<Account, ServerFnError> {
    if payload.email.len() > 254 || payload.password.len() > 128 || payload.display_name.len() > 100 {
        return Err(ServerFnError::new("Invalid input"));
    }

    let email = payload.email.to_lowercase();

    if auth_repository::find_account_by_email(&email).await?.is_some() {
        return Err(ServerFnError::new("Registration failed"));
    }

    let password_bytes = payload.password.into_bytes();
    let password_hash = tokio::task::spawn_blocking(move || {
        let salt = SaltString::generate(&mut OsRng);
        Argon2::default()
            .hash_password(&password_bytes, &salt)
            .map(|h| h.to_string())
    })
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    let account_id =
        auth_repository::create_account(&email, &password_hash, &payload.display_name)
            .await?;

    create_session_and_set_cookie(account_id).await?;

    let account = auth_repository::get_account_by_id(account_id)
        .await?
        .ok_or_else(|| ServerFnError::new("Account not found after creation"))?;

    Ok(account)
}

#[post("/api/v1/auth/login")]
pub async fn login(Json(payload): Json<LoginPayload>) -> Result<Account, ServerFnError> {
    if payload.email.len() > 254 || payload.password.len() > 128 {
        return Err(ServerFnError::new("Invalid email or password"));
    }

    let email = payload.email.to_lowercase();

    let account_with_hash = auth_repository::find_account_by_email(&email)
        .await?
        .ok_or_else(|| ServerFnError::new("Invalid email or password"))?;

    if is_account_locked(account_with_hash.locked_until, Utc::now().naive_utc()) {
        return Err(ServerFnError::new("Account temporarily locked. Try again later."));
    }

    let hash_str = account_with_hash.password_hash.clone();
    let password_bytes = payload.password.into_bytes();
    let verified = tokio::task::spawn_blocking(move || {
        PasswordHash::new(&hash_str)
            .map_err(|e| e.to_string())
            .and_then(|parsed| {
                Argon2::default()
                    .verify_password(&password_bytes, &parsed)
                    .map_err(|e| e.to_string())
            })
    })
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    if verified.is_err() {
        auth_repository::increment_failed_login(account_with_hash.id)
            .await
            .map_err(|e| {
                eprintln!("ERROR: Failed to increment failed_login_count: {}", e);
                e
            })?;
        return Err(ServerFnError::new("Invalid email or password"));
    }

    auth_repository::reset_failed_login(account_with_hash.id)
        .await
        .map_err(|e| {
            eprintln!("ERROR: Failed to reset failed_login_count: {}", e);
            e
        })?;
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

    let secure_attr =
        if std::env::var("COOKIE_SECURE").as_deref() == Ok("false") { "" } else { "; Secure" };
    let clear_cookie_value = format!(
        "session_id=; HttpOnly; SameSite=Lax; Path=/; Max-Age=0{}",
        secure_attr
    );
    let clear_cookie = HeaderValue::from_str(&clear_cookie_value)
        .map_err(|e| ServerFnError::new(e.to_string()))?;
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
        if std::env::var("COOKIE_SECURE").as_deref() == Ok("false") { "" } else { "; Secure" };

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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};

    fn now() -> NaiveDateTime {
        Utc::now().naive_utc()
    }

    #[test]
    fn not_locked_when_no_locked_until() {
        assert!(!is_account_locked(None, now()));
    }

    #[test]
    fn not_locked_when_locked_until_in_past() {
        let past = now() - Duration::minutes(1);
        assert!(!is_account_locked(Some(past), now()));
    }

    #[test]
    fn locked_when_locked_until_in_future() {
        let future = now() + Duration::minutes(14);
        assert!(is_account_locked(Some(future), now()));
    }

    #[test]
    fn not_locked_exactly_at_expiry() {
        // locked_until == now: not strictly greater, so not locked
        let t = now();
        assert!(!is_account_locked(Some(t), t));
    }
}
