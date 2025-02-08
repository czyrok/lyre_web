use axum::extract::FromRef;
use leptos::config::{get_configuration, LeptosOptions};

use crate::project::services::{
    project::ProjectService, project_context::ProjectContextService,
    project_slug::ProjectSlugService,
};

// Derive FromRef to allow multiple items in state, using Axum’s SubStates pattern.
#[derive(FromRef, Debug, Clone)]
pub struct AppState {
    pub options: LeptosOptions,
    pub project_service: ProjectService,
    pub project_context_service: ProjectContextService,
    pub project_slug_service: ProjectSlugService,
}

impl Default for AppState {
    fn default() -> Self {
        let configuration = get_configuration(None).unwrap();

        Self {
            options: configuration.leptos_options,
            project_service: ProjectService::default(),
            project_context_service: ProjectContextService::default(),
            project_slug_service: ProjectSlugService::default(),
        }
    }
}
