use std::sync::Arc;

use axum::{body::Body, extract::Request};
use http::request::Parts;
use hydration_context::SsrSharedContext;
use leptos::{
    prelude::{provide_context, Owner},
    IntoView,
};
use leptos_axum::{
    generate_route_list_with_exclusions_and_ssg_and_context, redirect,
    AxumRouteListing, ResponseOptions, StaticRouteGenerator,
};
use leptos_meta::ServerMetaContext;
use leptos_router::{
    components::provide_server_redirect, location::RequestUrl,
    static_routes::ResolvedStaticPath, RouteList,
};

use super::{super::state::app_state::AppState, shell::shell};

pub fn get_static_route_generator(
    app_state: AppState,
) -> (Vec<AxumRouteListing>, StaticRouteGenerator) {
    generate_route_list_with_exclusions_and_ssg_and_context(
        move || shell(app_state.options.clone()),
        vec![].into(),
        move || {
            provide_context(app_state.environment.clone());
            provide_context(app_state.project_service.clone());
            provide_context(app_state.project_context_service.clone());
            provide_context(app_state.project_slug_service.clone());
            provide_context(app_state.project_tag_service.clone());
        },
    )
}

pub async fn get_static_paths(app_state: AppState) -> Vec<ResolvedStaticPath> {
    generate_static_paths(
        move || shell(app_state.options.clone()),
        move || {
            provide_context(app_state.environment.clone());
            provide_context(app_state.project_service.clone());
            provide_context(app_state.project_context_service.clone());
            provide_context(app_state.project_slug_service.clone());
            provide_context(app_state.project_tag_service.clone());
        },
    )
    .await
}

/**
 * Method not exported from `leptos_router`
 */
fn provide_contexts(
    path: &str,
    meta_context: &ServerMetaContext,
    parts: Parts,
    default_res_options: ResponseOptions,
) {
    provide_context(RequestUrl::new(path));
    provide_context(meta_context.clone());
    provide_context(parts);
    provide_context(default_res_options);
    provide_server_redirect(redirect);
    leptos::nonce::provide_nonce();
}

/**
 * Based on `generate_route_list_with_exclusions_and_ssg_and_context` from `leptos_router`
 */
async fn generate_static_paths<IV>(
    app_fn: impl Fn() -> IV + Clone + Send + 'static,
    additional_context: impl Fn() + Clone + Send + 'static,
) -> Vec<ResolvedStaticPath>
where
    IV: IntoView + 'static,
{
    let owner = Owner::new_root(Some(Arc::new(SsrSharedContext::new())));

    let routes = owner
        .with(|| {
            // stub out a path for now
            provide_context(RequestUrl::new(""));
            let (mock_parts, _) = Request::new(Body::from("")).into_parts();
            let (mock_meta, _) = ServerMetaContext::new();
            provide_contexts("", &mock_meta, mock_parts, Default::default());
            additional_context();
            RouteList::generate(&app_fn)
        })
        .unwrap_or_default();

    let static_paths = routes.into_static_paths().await;

    static_paths
}
