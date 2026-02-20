// Round to 2 decimal places for currency
pub fn round_currency(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}

/// Extract the current account_id from the session cookie.
/// Returns None if no valid session is found.
#[cfg(feature = "server")]
pub async fn get_current_account_id() -> Option<uuid::Uuid> {
    use dioxus_fullstack::FullstackContext;

    let ctx = FullstackContext::current()?;

    // Extract and clone the cookie string while the guard is held, then release it
    let cookie_str: Option<String> = {
        let parts = ctx.parts_mut();
        parts
            .headers
            .get("cookie")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_owned())
    };

    let session_id_str = cookie_str?
        .split(';')
        .map(|s| s.trim().to_owned())
        .find(|s| s.starts_with("session_id="))
        .and_then(|s| s.strip_prefix("session_id=").map(|v| v.to_owned()))?;

    let session_id: uuid::Uuid = session_id_str.parse().ok()?;

    crate::auth::auth_repository::get_session_account_id(session_id).await.ok()?
}
