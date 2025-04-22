use axum::extract::FromRef;
use leptos::config::LeptosOptions;

use super::environment_context::EnvironmentContext;
use crate::project::{
    repositories::{
        project_context_repository::ProjectContextRepository,
        project_link_repository::ProjectLinkRepository,
        project_repository::ProjectRepository,
        project_slug_repository::ProjectSlugRepository,
        project_tag_repository::ProjectTagRepository,
    },
    services::{
        project_context_service::ProjectContextService,
        project_service::ProjectService,
        project_slug_service::ProjectSlugService,
        project_tag_service::ProjectTagService,
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
    pub project_tag_service: ProjectTagService,
}

impl AppState {
    pub fn new(
        environment: EnvironmentContext,
        options: LeptosOptions,
    ) -> Self {
        let project_repository = ProjectRepository::new(environment.clone());
        let project_tag_repository =
            ProjectTagRepository::new(environment.clone());
        let project_context_repository =
            ProjectContextRepository::new(environment.clone());
        let project_slug_repository =
            ProjectSlugRepository::new(environment.clone());
        let project_link_repository = ProjectLinkRepository::default();

        let project_service = ProjectService::new(
            project_repository,
            project_tag_repository.clone(),
            project_link_repository,
        );
        let project_context_service =
            ProjectContextService::new(project_context_repository);
        let project_slug_service =
            ProjectSlugService::new(project_slug_repository);

        let project_tag_service =
            ProjectTagService::new(project_tag_repository);

        Self {
            environment,
            options,
            project_service,
            project_context_service,
            project_slug_service,
            project_tag_service,
        }
    }
}
