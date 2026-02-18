use dioxus::prelude::*;
use super::tricount_models::{TricountApiResponse, TricountRegistry, TricountResponseItem};

const TRICOUNT_API_BASE: &str = "https://api.tricount.bunq.com";
const USER_AGENT: &str = "com.bunq.tricount.android:RELEASE:7.0.7:3174:ANDROID:13:C";

pub fn extract_tricount_key(input: &str) -> String {
    let trimmed = input.trim();
    if trimmed.contains('/') {
        trimmed
            .rsplit('/')
            .find(|s| !s.is_empty())
            .unwrap_or(trimmed)
            .to_string()
    } else {
        trimmed.to_string()
    }
}

#[cfg(feature = "server")]
struct AuthResult {
    token: String,
    user_id: i64,
}

#[cfg(feature = "server")]
async fn authenticate(client: &reqwest::Client, app_uuid: &str) -> Result<AuthResult, ServerFnError> {
    use rsa::pkcs8::EncodePublicKey;
    use rsa::RsaPrivateKey;

    let mut rng = rand::thread_rng();
    let private_key = RsaPrivateKey::new(&mut rng, 2048)
        .map_err(|e| ServerFnError::new(format!("RSA key generation failed: {e}")))?;
    let public_key_pem = private_key
        .to_public_key()
        .to_public_key_pem(rsa::pkcs8::LineEnding::LF)
        .map_err(|e| ServerFnError::new(format!("PEM encoding failed: {e}")))?;

    let body = serde_json::json!({
        "app_installation_uuid": app_uuid,
        "client_public_key": public_key_pem,
        "device_description": "Android"
    });

    let resp = client
        .post(format!("{TRICOUNT_API_BASE}/v1/session-registry-installation"))
        .header("User-Agent", USER_AGENT)
        .header("app-id", app_uuid)
        .header("X-Bunq-Client-Request-Id", uuid::Uuid::new_v4().to_string())
        .json(&body)
        .send()
        .await
        .map_err(|e| ServerFnError::new(format!("Tricount auth request failed: {e}")))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        return Err(ServerFnError::new(format!(
            "Tricount authentication failed ({status}): {text}"
        )));
    }

    let json: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to parse auth response: {e}")))?;

    let response_arr = json["Response"]
        .as_array()
        .ok_or_else(|| ServerFnError::new("Invalid auth response format".to_string()))?;

    let token = response_arr
        .iter()
        .find_map(|item| {
            item.get("Token")
                .and_then(|t| t.get("token"))
                .and_then(|t| t.as_str())
        })
        .ok_or_else(|| ServerFnError::new("No auth token in Tricount response".to_string()))?
        .to_string();

    // Try multiple user types — anonymous sessions may use UserRegistryAnonymous
    let user_id = response_arr
        .iter()
        .find_map(|item| {
            ["UserPerson", "UserRegistryAnonymous", "UserCompany", "UserApiKey"]
                .iter()
                .find_map(|key| {
                    item.get(key)
                        .and_then(|u| u.get("id"))
                        .and_then(|id| id.as_i64())
                })
        })
        .ok_or_else(|| {
            // Dump raw response to help diagnose
            let keys: Vec<String> = response_arr
                .iter()
                .flat_map(|item| item.as_object().map(|o| o.keys().cloned().collect::<Vec<_>>()).unwrap_or_default())
                .collect();
            ServerFnError::new(format!(
                "No user ID in Tricount auth response. Response item keys: {:?}",
                keys
            ))
        })?;

    Ok(AuthResult { token, user_id })
}

#[cfg(feature = "server")]
pub async fn fetch_tricount(tricount_key: &str) -> Result<TricountRegistry, ServerFnError> {
    let client = reqwest::Client::new();
    let app_uuid = uuid::Uuid::new_v4().to_string();
    let auth = authenticate(&client, &app_uuid).await?;

    let url = format!(
        "{TRICOUNT_API_BASE}/v1/user/{}/registry",
        auth.user_id
    );

    let resp = client
        .get(&url)
        .query(&[("public_identifier_token", tricount_key)])
        .header("User-Agent", USER_AGENT)
        .header("app-id", &app_uuid)
        .header("X-Bunq-Client-Authentication", &auth.token)
        .header("X-Bunq-Client-Request-Id", uuid::Uuid::new_v4().to_string())
        .send()
        .await
        .map_err(|e| ServerFnError::new(format!("Tricount API request failed: {e}")))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        return Err(ServerFnError::new(format!(
            "Tricount introuvable ou erreur API ({status}) — URL: {url}?public_identifier_token={tricount_key} — {text}"
        )));
    }

    let api_response: TricountApiResponse = resp
        .json()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to parse Tricount data: {e}")))?;

    let registry = api_response
        .response
        .into_iter()
        .find_map(|item| match item {
            TricountResponseItem::Registry { registry } => Some(registry),
            _ => None,
        })
        .ok_or_else(|| ServerFnError::new("No Registry found in Tricount response".to_string()))?;

    Ok(registry)
}
