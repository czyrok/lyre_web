use axum::{
    body::Body as AxumBody,
    extract::{Path, State},
    http::Request,
    response::IntoResponse,
};
use leptos::{logging::log, prelude::*};
use leptos_axum::handle_server_fns_with_context;

use super::super::state::app_state::AppState;

pub async fn server_function_handler(
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
            provide_context(app_state.project_tag_service.clone());
        },
        request,
    )
    .await
}
