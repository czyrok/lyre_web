use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "ssr")] {
    use axum::extract::FromRef;
    use leptos::config::LeptosOptions;
    use std::error::Error;

    use crate::project::data::project_service::ProjectService;
    use crate::project::data::project_repository::ProjectRepository;


    // Derive FromRef to allow multiple items in state, using Axum’s SubStates pattern.
    #[derive(FromRef, Debug, Clone)]
    pub struct AppState {
        pub options: LeptosOptions,
        pub project_service: ProjectService,
    }

    impl AppState {
        pub async fn new(options: LeptosOptions) -> Result<Self, Box<dyn Error>> {
            let mut project_repository = ProjectRepository::new("project_data/");

            project_repository.cache_project_data().await?;

            let project_service = ProjectService::new(project_repository);

            Ok(
                Self {
                    options,
                    project_service,
                }
            )
        }
    }
}
}
