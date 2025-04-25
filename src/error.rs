// use axum::Json;
// use axum::http::StatusCode;
// use axum::response::{Response, IntoResponse};
// use serde_json::json;
// 
// 
// #[derive(Debug)]
// pub enum AppError {
//     Conflict(String),
//     InternalServerError,
//     // 其他错误类型...
// }
// 
// impl IntoResponse for AppError {
//     fn into_response(self) -> Response {
//         match self {
//             AppError::Conflict(msg) => {
//                 (StatusCode::CONFLICT, Json(json!({ "error": msg }))).into_response()
//             }
//             AppError::InternalServerError => (
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 Json(json!({ "error": "Internal server error" })),
//             )
//                 .into_response(),
//         }
//     }
// }
