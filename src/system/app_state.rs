use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "ssr")] {
    use axum::extract::FromRef;
    use leptos::config::LeptosOptions;

    use crate::project::data::project_service::ProjectService;

    // Derive FromRef to allow multiple items in state, using Axum’s SubStates pattern.
    #[derive(FromRef, Debug, Clone)]
    pub struct AppState {
        pub leptos_options: LeptosOptions,
        pub project_service: ProjectService,
    }
}
}
