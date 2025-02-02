use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "ssr")] {
    use axum::extract::FromRef;
    use leptos::config::LeptosOptions;
    use std::error::Error;

    use crate::project::data::project_service::ProjectService;

    // Derive FromRef to allow multiple items in state, using Axum’s SubStates pattern.
    #[derive(FromRef, Debug, Clone)]
    pub struct AppState {
        pub options: LeptosOptions,
        pub project_service: ProjectService,
    }

    impl AppState {
        // TODO:
        // async fn load_context() -> Result<Self, Box<dyn Error>> {
        //     // TODO: Mettre un fonction dans service + ici
        //     //project_services.1.cache_project_data().await?;
        // }

        pub async fn new(options: LeptosOptions) -> Result<Self, Box<dyn Error>> {
            Ok(
                Self {
                    options,
                    project_service: ProjectService::default(),
                }
            )
        }
    }
}
}
