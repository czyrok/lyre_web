mod app;
mod common;
mod home;
mod project;
mod system;

use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "ssr")] {
    use leptos_axum::generate_route_list_with_exclusions_and_ssg_and_context;
    use axum::{
        routing::get,
        Router,
    };
    use leptos::logging::log;
    use leptos::prelude::*;
    use project::data::{project_repository::ProjectRepository, project_service::ProjectService};
    use system::shell::shell;
    use std::error::Error;
    use system::app_state::AppState;
    use leptos_axum::{LeptosRoutes};
    use system::fallback::file_and_error_handler;
    use system::handlers::{server_fn_handler, leptos_routes_handler};

    #[tokio::main]
    async fn main() -> Result<(), Box<dyn Error>> {
        let conf = get_configuration(None).unwrap();
        let addr = conf.leptos_options.site_addr;
        let leptos_options = conf.leptos_options;

        let mut project_repository = ProjectRepository::new("project_data/");

        project_repository.cache_project_data().await?;

        let project_service = ProjectService::new(project_repository);
        let project_service_clone = project_service.clone();

        // Generate the list of routes in your Leptos App
        let (routes, static_routes) = generate_route_list_with_exclusions_and_ssg_and_context({
            let leptos_options = leptos_options.clone();

            move || shell(leptos_options.clone())
        }, vec![].into(), move || {
            provide_context(project_service_clone.clone());
        });

        static_routes.generate(&leptos_options).await;

        let app_state = AppState {
            leptos_options,
            project_service,
        };

        let app = Router::new()
            .route(
                "/api/*fn_name",
                get(server_fn_handler).post(server_fn_handler),
            )
            .leptos_routes_with_handler(routes, get(leptos_routes_handler))
            .fallback(file_and_error_handler)
            .with_state(app_state);

        log!("listening on http://{}", &addr);

        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

        axum::serve(listener, app.into_make_service())
            .await
            .unwrap();

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
