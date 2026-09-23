use std::{future::Future, pin::Pin};

use axum::{
    body::Body as AxumBody,
    extract::State,
    http::Request,
    response::{IntoResponse, Response},
};
use leptos::prelude::*;
use leptos_axum::AxumRouteListing;

use super::super::{route::shell::shell, state::app_state::AppState};

pub fn leptos_routes_handler(
    app_state: AppState,
    routes: Vec<AxumRouteListing>,
) -> impl Fn(
    State<AppState>,
    Request<AxumBody>,
) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>
       + Clone
       + Send
       + 'static {
    leptos_axum::render_route_with_context(
        routes,
        provide_app_contexts(app_state.clone()),
        move || shell(app_state.options.clone()),
    )
}

//// Kept apart from `leptos_routes_handler`, which panics without an Axum `MatchedPath`
pub async fn leptos_fallback_handler(
    State(app_state): State<AppState>,
    req: Request<AxumBody>,
) -> Response {
    let handler = leptos_axum::render_app_to_stream_with_context(
        provide_app_contexts(app_state.clone()),
        move || shell(app_state.options.clone()),
    );

    handler(req).await.into_response()
}

fn provide_app_contexts(
    app_state: AppState,
) -> impl Fn() + Clone + Send + Sync + 'static {
    move || {
        provide_context(app_state.environment.clone());
        provide_context(app_state.project_service.clone());
        provide_context(app_state.project_context_service.clone());
        provide_context(app_state.project_slug_service.clone());
        provide_context(app_state.project_tag_service.clone());
    }
}
