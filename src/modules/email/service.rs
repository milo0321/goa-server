use super::model::*;
use super::repository;

use crate::error::ApiError;
use crate::{
    common::pagination::{PaginatedResponse, PaginationParams},
    db::AppState,
};
use axum::http::StatusCode;
use uuid::Uuid;

pub async fn list_accounts(
    state: AppState,
    params: PaginationParams,
) -> Result<PaginatedResponse<EmailAccount>, ApiError> {
    let response = repository::list_accounts(state, params).await?;
    Ok(response)
}

pub async fn get_account(state: AppState, id: Uuid) -> Result<EmailAccount, ApiError> {
    let response = repository::get_account(&state, id).await?;
    Ok(response)
}

pub async fn create_account(
    state: AppState,
    params: CreateEmailAccount,
) -> Result<EmailAccount, ApiError> {
    let response: EmailAccount = repository::create_account(&state, params).await?;
    Ok(response)
}

pub async fn update_account(
    state: AppState,
    id: Uuid,
    params: UpdateEmailAccount,
) -> Result<EmailAccount, ApiError> {
    let response = repository::update_account(&state, id, params).await?;
    Ok(response)
}

pub async fn delete_account(state: AppState, id: Uuid) -> Result<StatusCode, ApiError> {
    repository::delete_account(&state, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn list_messages(
    state: AppState,
    params: PaginationParams,
) -> Result<PaginatedResponse<EmailMessage>, ApiError> {
    let response = repository::list_messages(&state, params).await?;
    Ok(response)
}

pub async fn get_message(state: AppState, id: Uuid) -> Result<EmailMessage, ApiError> {
    let response = repository::get_message(&state, id).await?;
    Ok(response)
}

pub async fn create_message(
    state: AppState,
    params: CreateEmailMessage,
) -> Result<EmailMessage, ApiError> {
    let response: EmailMessage = repository::create_message(&state, params).await?;
    Ok(response)
}

pub async fn update_message(
    state: AppState,
    id: Uuid,
    params: UpdateEmailMessage,
) -> Result<EmailMessage, ApiError> {
    let response = repository::update_message(&state, id, params).await?;
    Ok(response)
}

pub async fn delete_message(state: AppState, id: Uuid) -> Result<StatusCode, ApiError> {
    repository::delete_message(&state, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// pub async fn store_and_attachments(
//     pool: &PgPool,
//     config_id: i64,
//     subject: String,
//     sender: String,
//     received_at: chrono::NaiveDateTime,
//     body: String,
//     attachments: Vec<(String, Vec<u8>, String)>, // (filename, filedata, mimetype)
// ) -> Result<(), anyhow::Error> {
//     let email = EmailMessage {
//         id: 0,
//         config_id,
//         subject,
//         sender,
//         received_at,
//         body,
//     };
//
//     let email_id = repository::save_email(pool, &email).await?;
//
//     for (filename, data, mimetype) in attachments {
//         let uuid = Uuid::new_v4();
//         let path = format!("./attachments/{}_{}", uuid, filename);
//
//         fs::write(&path, &data)?;
//
//         let attachment = EmailAttachment {
//             id: 0,
//             email_id,
//             filename,
//             filepath: path,
//             mimetype,
//         };
//
//         repository::save_attachment(pool, &attachment).await?;
//     }
//
//     Ok(())
// }

pub async fn fetch_emails(state: AppState) {
    let param = PaginationParams {
        page: Some(1),
        limit: Some(100),
    };
    let accounts = match repository::list_accounts(state, param).await {
        Ok(list) => list,
        Err(e) => {
            tracing::error!("Fetch configs failed: {}", e);
            return;
        }
    };
    //
    // for config in configs {
    //     // 模拟从服务器拉邮件
    //     let email = EmailMessage {
    //         id: 0,
    //         config_id: config.id,
    //         subject: "Hello from IMAP!".to_string(),
    //         sender: "noreply@example.com".to_string(),
    //         received_at: Utc::now().naive_utc(),
    //         body: "Test Body".to_string(),
    //     };
    //
    //     match repository::save_email(pool, &email).await {
    //         Ok(email_id) => {
    //             // 可选：保存附件
    //             let attachment = EmailAttachment {
    //                 id: 0,
    //                 email_id,
    //                 filename: "doc.txt".to_string(),
    //                 filepath: "/data/email/doc.txt".to_string(),
    //                 mimetype: "text/plain".to_string(),
    //             };
    //             let _ = repository::save_attachment(pool, &attachment).await;
    //         }
    //         Err(e) => tracing::error!("Save email failed: {}", e),
    //     }
    // }
}
