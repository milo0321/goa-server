use crate::common::router::resource_router;
use crate::db::AppState;
use crate::modules::email::controller::*;
use axum::routing::{delete, post, put};
use axum::{Router, routing::get};

pub fn email_routes() -> Router<AppState> {
    let accounts_router = resource_router(
        "emails/accounts",
        get(list_accounts),
        post(create_account),
        get(get_account),
        put(update_account),
        delete(delete_account),
    );

    let mails_router = resource_router(
        "emails",
        get(list_messages),
        post(create_message),
        get(get_message),
        put(update_message),
        delete(delete_message),
    );

    Router::new().merge(accounts_router).merge(mails_router)
}
