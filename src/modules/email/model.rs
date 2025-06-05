use axum::Json;
use axum::response::{IntoResponse, Response};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct EmailAccount {
    pub id: i64,
    pub email_address: String,
    pub imap_server: String,
    pub imap_port: i32,
    pub username: String,
    pub password: String,
    pub use_ssl: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl IntoResponse for EmailAccount {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct EmailAccountFields {
    pub email_address: String,
    pub imap_server: String,
    pub imap_port: i32,
    pub username: String,
    pub password: String,
    pub use_ssl: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateEmailAccount {
    #[serde(flatten)]
    pub fields: EmailAccountFields,
}

impl IntoResponse for CreateEmailAccount {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateEmailAccount {
    pub id: i64,
    #[serde(flatten)]
    pub fields: EmailAccountFields,
}

impl IntoResponse for UpdateEmailAccount {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmailMessageFields {
    pub id: i64,
    pub config_id: i64,
    pub subject: String,
    pub sender: String,
    pub received_at: DateTime<Utc>,
    pub body: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct EmailMessage {
    pub id: i64,
    #[serde(flatten)]
    pub fields: EmailMessageFields,
}

impl IntoResponse for EmailMessage {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CreateEmailMessage {
    #[serde(flatten)]
    pub fields: EmailMessageFields,
}

impl IntoResponse for CreateEmailMessage {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UpdateEmailMessage {
    pub id: i64,
    #[serde(flatten)]
    pub fields: EmailMessageFields,
}

impl IntoResponse for UpdateEmailMessage {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct EmailAttachment {
    pub id: i64,
    pub email_id: i64,
    pub filename: String,
    pub filepath: String,
    pub mimetype: String,
}

impl IntoResponse for EmailAttachment {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
