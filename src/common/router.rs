use axum::{Router, handler::Handler, routing::get};
use tower_http::trace::TraceLayer;
use tracing::Level;

pub fn resource_router<T, S, ListF, CreateF, RetrieveF, UpdateF, DeleteF>(
    prefix: &str,
    list: ListF,
    create: CreateF,
    retrieve: RetrieveF,
    update: UpdateF,
    delete: DeleteF,
) -> Router<S>
where
    T: 'static,
    S: Clone + Send + Sync + 'static,
    ListF: Handler<T, S>,
    RetrieveF: Handler<T, S>,
    CreateF: Handler<T, S>,
    UpdateF: Handler<T, S>,
    DeleteF: Handler<T, S>,
{
    let clean_prefix = prefix.trim_start_matches('/').trim_end_matches('/');
    let base = format!("/{}", clean_prefix); // /customers
    let detail = format!("/{}/{{id}}", clean_prefix); // /customers/{id}

    Router::new()
        .route(&base, get(list).post(create))
        .route(&detail, get(retrieve).put(update).delete(delete))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(tower_http::trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(tower_http::trace::DefaultOnResponse::new().level(Level::INFO)),
        )
}
