use leptos::{prelude::ServerFnError, server, server_fn::codec::GetUrl};

use crate::{
    common::error::server_error_dto::ServerErrorDto,
    project::dto::project::ProjectDto,
};

#[server(prefix = "/api", endpoint = "projects", input = GetUrl)]
pub async fn get_project(
    slug: String,
) -> Result<ProjectDto, ServerFnError<ServerErrorDto>> {
    use crate::{
        project::use_cases::get_project::GetProjectUseCase,
        system::{context::use_project_service, use_case_runner::run_use_case},
    };

    let project_service = use_project_service()?;

    let use_case = GetProjectUseCase::new(project_service);

    run_use_case(use_case, slug).await
}
