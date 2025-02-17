use axum::{
    body::Body as AxumBody,
    extract::{Path, State},
    http::Request,
    response::{IntoResponse, Response},
};
use leptos::{logging::log, prelude::*};
use leptos_axum::handle_server_fns_with_context;

use super::{shell::shell, state::app_state::AppState};

pub async fn server_fn_handler(
    State(app_state): State<AppState>,
    path: Path<String>,
    request: Request<AxumBody>,
) -> impl IntoResponse {
    log!("{:?}", path);

    handle_server_fns_with_context(
        move || {
            provide_context(app_state.environment.clone());
            provide_context(app_state.options.clone());
            provide_context(app_state.project_service.clone());
            provide_context(app_state.project_context_service.clone());
            provide_context(app_state.project_slug_service.clone());
        },
        request,
    )
    .await
}

pub async fn leptos_routes_handler(
    State(app_state): State<AppState>,
    req: Request<AxumBody>,
) -> Response {
    let handler = leptos_axum::render_app_to_stream_with_context(
        move || {
            provide_context(app_state.environment.clone());
            provide_context(app_state.project_service.clone());
            provide_context(app_state.project_context_service.clone());
            provide_context(app_state.project_slug_service.clone());
        },
        move || shell(app_state.options.clone()),
    );

    handler(req).await.into_response()
}
