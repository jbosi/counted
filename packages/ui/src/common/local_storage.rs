use serde::{Deserialize, Serialize};
use uuid::Uuid;

const LS_KEY: &str = "counted_local_storage";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct LocalStorageState {
    pub projects: Vec<LocalStorageProject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocalStorageProject {
    pub project_id: Uuid,
    pub user_id: Option<i32>,
}

pub fn read_from_ls() -> LocalStorageState {
    #[cfg(target_arch = "wasm32")]
    {
        let result = (|| -> Option<LocalStorageState> {
            let storage = web_sys::window()?.local_storage().ok()??;
            let raw = storage.get_item(LS_KEY).ok()??;
            serde_json::from_str(&raw).ok()
        })();
        return result.unwrap_or_default();
    }
    #[cfg(not(target_arch = "wasm32"))]
    LocalStorageState::default()
}

pub fn write_to_ls(state: &LocalStorageState) {
    #[cfg(target_arch = "wasm32")]
    {
        let _ = (|| -> Option<()> {
            let storage = web_sys::window()?.local_storage().ok()??;
            let json = serde_json::to_string(state).ok()?;
            storage.set_item(LS_KEY, &json).ok()?;
            Some(())
        })();
    }
    #[cfg(not(target_arch = "wasm32"))]
    let _ = state;
}

/// Upsert a project entry. Does not overwrite an existing user_id with None.
pub fn upsert_project(state: &mut LocalStorageState, project_id: Uuid, user_id: Option<i32>) {
    if let Some(entry) = state.projects.iter_mut().find(|p| p.project_id == project_id) {
        if user_id.is_some() {
            entry.user_id = user_id;
        }
    } else {
        state.projects.push(LocalStorageProject { project_id, user_id });
    }
}

pub fn user_color_class(user_id: i32) -> &'static str {
    const COLORS: &[&str] = &[
        "bg-primary",
        "bg-secondary",
        "bg-accent",
        "bg-info",
        "bg-success",
        "bg-warning",
        "bg-error",
    ];
    COLORS[(user_id.unsigned_abs() as usize) % COLORS.len()]
}

/// Returns up to 2 uppercase initials from a name.
pub fn initials(name: &str) -> String {
    let mut parts = name.split_whitespace();
    match (parts.next(), parts.next()) {
        (Some(first), Some(last)) => format!(
            "{}{}",
            first.chars().next().unwrap_or_default().to_uppercase(),
            last.chars().next().unwrap_or_default().to_uppercase()
        ),
        (Some(first), None) => first
            .chars()
            .next()
            .unwrap_or_default()
            .to_uppercase()
            .to_string(),
        _ => "?".to_string(),
    }
}
