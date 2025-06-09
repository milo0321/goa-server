use axum::Json;
use axum::response::{IntoResponse, Response};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, FromRow, Type)]
pub struct Supplier {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    #[sqlx(default)]
    pub phone: Option<String>,
    #[sqlx(default)]
    pub company: Option<String>,
    #[sqlx(default)]
    pub position: Option<String>,
    #[sqlx(default)]
    pub address: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl IntoResponse for Supplier {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct CreateSupplier {
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub company: Option<String>,
    pub position: Option<String>,
    pub address: Option<String>,
}

impl IntoResponse for CreateSupplier {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSupplier {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub company: Option<String>,
    pub position: Option<String>,
    pub address: Option<String>,
}

impl IntoResponse for UpdateSupplier {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
