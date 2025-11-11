use leptos::prelude::{Get, Resource, Signal};

use crate::{
    core::{
        data::fetch_state::FetchState,
        error::server_function_error::ServerFunctionError,
    },
    project::{
        api::project_api::get_project, data::project::Project,
        dto::project_dto::ProjectDto,
    },
};

#[derive(Clone, Copy)]
pub struct ProjectDetailsResource(
    Resource<Result<ProjectDto, ServerFunctionError>>,
);

impl ProjectDetailsResource {
    pub fn new(project_slug: Signal<String>) -> Self {
        let resource =
            Resource::new_blocking(move || project_slug.get(), get_project);

        Self(resource)
    }

    pub async fn get_project(&self) -> Result<Project, FetchState> {
        self.0
            .await
            .map(|project_dto| project_dto.project)
            .map_err(FetchState::Errored)
    }
}
