mod app;
mod common;
mod home;
mod project;
mod system;

use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "ssr")] {
    use axum::{
        routing::get,
        Router,
    };
    use leptos::prelude::*;
    use std::error::Error;
    use system::app_state::AppState;
    use leptos_axum::{LeptosRoutes};
    use system::fallback::file_and_error_handler;
    use system::static_route_generator::get_static_route_generator;
    use system::handlers::{server_fn_handler, leptos_routes_handler};
    use project::use_cases::refresh_project_cache::RefreshProjectCacheUseCase;
    use common::use_case::UseCase;
    use system::environment_context::EnvironmentContext;

    fn get_app_state() -> Result<AppState, Box<dyn Error>> {
        let environment = EnvironmentContext::load_environment()?;
        let configuration = get_configuration(None)?;

        Ok(AppState::new(environment, configuration.leptos_options))
    }

    async fn refresh_project_cache(app_state: AppState) -> Result<(), Box<dyn Error>> {
        let mut use_case = RefreshProjectCacheUseCase::new(app_state.project_service);

        use_case.run(()).await?;

        Ok(())
    }

    async fn serve_leptos(app_state: AppState) {
        // Generate the list of routes in your Leptos App
        let (routes, static_routes) = get_static_route_generator(app_state.clone());

        static_routes.generate(&app_state.options).await;

        let app = Router::new()
            .route(
                "/api/*fn_name",
                get(server_fn_handler).post(server_fn_handler),
            )
            .leptos_routes_with_handler(routes, get(leptos_routes_handler))
            .fallback(file_and_error_handler)
            .with_state(app_state.clone());

        let address = app_state.options.site_addr;

        let listener = tokio::net::TcpListener::bind(address).await.unwrap();

        axum::serve(listener, app.into_make_service())
            .await
            .unwrap();
    }

    #[tokio::main]
    async fn main() -> Result<(), Box<dyn Error>> {
        let app_state = get_app_state()?;

        refresh_project_cache(app_state.clone()).await?;

        serve_leptos(app_state).await;

        Ok(())
    }
} else {
    pub fn main() {
        // no client-side main function
        // unless we want this to work with e.g., Trunk for pure client-side testing
        // see lib.rs for hydration function instead
    }
}
}
