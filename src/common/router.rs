use axum::{Router, http::StatusCode, routing::MethodRouter};
use tower_http::trace::TraceLayer;

pub fn resource_router<S>(
    list: MethodRouter<S>,
    create: MethodRouter<S>,
    retrieve: MethodRouter<S>,
    update: MethodRouter<S>,
    delete: MethodRouter<S>,
) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", list.post(create).options(|| async { StatusCode::OK }))
        .route(
            "/{id}",
            retrieve
                .put(update)
                .delete(delete)
                .options(|| async { StatusCode::OK }),
        )
        .layer(TraceLayer::new_for_http())
}

pub fn resource_router_with_prefix<S>(
    prefix: &str,
    list: MethodRouter<S>,
    create: MethodRouter<S>,
    retrieve: MethodRouter<S>,
    update: MethodRouter<S>,
    delete: MethodRouter<S>,
) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new().nest(
        prefix,
        resource_router(list, create, retrieve, update, delete),
    )
}
