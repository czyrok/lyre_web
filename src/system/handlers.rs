use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "ssr")] {
    use axum::{body::Body as AxumBody, extract::State};
    use axum::{
        extract::Path,
        http::Request,
        response::{IntoResponse, Response},
    };
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::handle_server_fns_with_context;

    use super::{app_state::AppState, shell::shell};

    pub async fn server_fn_handler(
        State(app_state): State<AppState>,
        path: Path<String>,
        request: Request<AxumBody>,
    ) -> impl IntoResponse {
        log!("{:?}", path);

        handle_server_fns_with_context(
            move || {
                provide_context(app_state.leptos_options.clone());
                provide_context(app_state.project_service.clone());
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
                provide_context(app_state.project_service.clone());
            },
            move || shell(app_state.leptos_options.clone()),
        );

        handler(req).await.into_response()
    }
}
}
