use crate::users::get_user_by_id;
use anyhow::anyhow;
#[cfg(feature = "server")]
use async_trait::async_trait;
#[cfg(feature = "server")]
use axum::response::{IntoResponse, Response};
#[cfg(feature = "server")]
use axum_session_auth::{AuthSession, Authentication};
#[cfg(feature = "server")]
use axum_session_sqlx::SessionPgPool;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
#[cfg(feature = "server")]
use sqlx::postgres::PgPool;
#[cfg(feature = "server")]
use sqlx::FromRow;
use std::str::FromStr;

#[cfg_attr(feature = "server", derive(FromRow, Clone))]
pub struct SqlPermissionTokens {
    pub token: String,
}

// Shared ?

#[derive(Debug)]
pub struct AuthSessionLayerNotFound;

impl std::fmt::Display for AuthSessionLayerNotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AuthSessionLayer was not found")
    }
}

impl std::error::Error for AuthSessionLayerNotFound {}

#[cfg(feature = "server")]
impl IntoResponse for AuthSessionLayerNotFound {
    fn into_response(self) -> Response {
        (http::status::StatusCode::INTERNAL_SERVER_ERROR, "AuthSessionLayer was not found")
            .into_response()
    }
}

#[cfg(feature = "server")]
#[async_trait]
impl<S: std::marker::Sync + std::marker::Send> axum::extract::FromRequestParts<S> for Session {
    type Rejection = AuthSessionLayerNotFound;

    async fn from_request_parts(
        parts: &mut http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        AuthSession::<User, i32, SessionPgPool, PgPool>::from_request_parts(parts, state)
            .await
            .map(Session)
            .map_err(|_| AuthSessionLayerNotFound)
    }
}

// User

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[cfg_attr(feature = "server", derive(FromRow))]
pub struct User {
    pub id: i32,
    pub anonymous: bool,
    pub name: String,
    pub balance: Option<f64>,
    pub created_at: Option<NaiveDateTime>,
    pub password: Option<String>,
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: 1,
            anonymous: true,
            name: "Guest".into(),
            password: Some(String::new()),
            balance: Some(f64::default()),
            created_at: Some(NaiveDateTime::default()),
        }
    }
}

#[cfg(feature = "server")]
#[async_trait]
impl Authentication<User, i32, PgPool> for User {
    async fn load_user(userid: i32, pool: Option<&PgPool>) -> Result<User, anyhow::Error> {
        get_user_by_id(userid).await.map_err(|_| anyhow!("Could not load user"))
    }

    fn is_authenticated(&self) -> bool {
        !self.anonymous
    }

    fn is_active(&self) -> bool {
        !self.anonymous
    }

    fn is_anonymous(&self) -> bool {
        self.anonymous
    }
}

#[cfg(feature = "server")]
pub struct Session(pub AuthSession<User, i32, SessionPgPool, PgPool>);
