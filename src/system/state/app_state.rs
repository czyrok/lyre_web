use axum::extract::FromRef;
use leptos::config::LeptosOptions;

use super::environment_context::EnvironmentContext;
use crate::project::{
    repositories::{
        project::ProjectRepository, project_context::ProjectContextRepository,
        project_slug::ProjectSlugRepository, project_tag::ProjectTagRepository,
    },
    services::{
        project::ProjectService, project_context::ProjectContextService,
        project_slug::ProjectSlugService,
    },
};

// Derive FromRef to allow multiple items in state, using Axum’s SubStates pattern.
#[derive(FromRef, Debug, Clone)]
pub struct AppState {
    pub environment: EnvironmentContext,
    pub options: LeptosOptions,
    pub project_service: ProjectService,
    pub project_context_service: ProjectContextService,
    pub project_slug_service: ProjectSlugService,
}

impl AppState {
    pub fn new(
        environment: EnvironmentContext,
        options: LeptosOptions,
    ) -> Self {
        let project_repository = ProjectRepository::new(environment.clone());
        let project_tag_repository = ProjectTagRepository::default();
        let project_context_repository =
            ProjectContextRepository::new(environment.clone());
        let project_slug_repository =
            ProjectSlugRepository::new(environment.clone());

        let project_service =
            ProjectService::new(project_repository, project_tag_repository);
        let project_context_service =
            ProjectContextService::new(project_context_repository);
        let project_slug_service =
            ProjectSlugService::new(project_slug_repository);

        Self {
            environment,
            options,
            project_service,
            project_context_service,
            project_slug_service,
        }
    }
}
