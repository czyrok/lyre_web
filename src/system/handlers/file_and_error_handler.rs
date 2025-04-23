use axum::{
    body::Body,
    extract::State,
    http::{Request, Response, StatusCode, Uri},
    response::{IntoResponse, Redirect, Response as AxumResponse},
};
use leptos::prelude::*;
use tower::ServiceExt;
use tower_http::services::{fs::ServeFileSystemResponseBody, ServeDir};

use super::leptos_route_handler::leptos_routes_handler;
use crate::{
    shared::layouts::not_found_error_page_layout::NotFoundErrorPageLayout,
    system::state::app_state::AppState,
};

pub async fn file_and_error_handler(
    uri: Uri,
    app_state: State<AppState>,
    req: Request<Body>,
) -> AxumResponse {
    let root = app_state.options.site_root.clone();
    let res = get_static_file(uri.clone(), &root).await.unwrap();

    if res.status() == StatusCode::OK {
        res.into_response()
    } else {
        //// If the static file is not found, it render the app directly
        leptos_routes_handler(app_state, req).await
    }
}

async fn get_static_file(
    uri: Uri,
    root: &str,
) -> Result<Response<ServeFileSystemResponseBody>, (StatusCode, String)> {
    let req = Request::builder()
        .uri(uri.clone())
        .body(Body::empty())
        .unwrap();
    // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
    // This path is relative to the cargo root
    match ServeDir::new(root).oneshot(req).await {
        Ok(res) => Ok(res.map(|b| b)),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {err}"),
        )),
    }
}
