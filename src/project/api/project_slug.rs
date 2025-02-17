use leptos::{prelude::ServerFnError, server, server_fn::codec::GetUrl};

use crate::{
    common::error::server_error_dto::ServerErrorDto,
    project::dto::project_slugs::ProjectSlugsDto,
};

#[server(prefix = "/api", endpoint = "project_slugs", input = GetUrl)]
pub async fn get_project_slugs(
) -> Result<ProjectSlugsDto, ServerFnError<ServerErrorDto>> {
    use crate::{
        project::use_cases::get_project_slugs::GetProjectSlugsUseCase,
        system::{
            state::contexts::use_project_slug_service,
            use_case_runner::run_use_case,
        },
    };

    let project_slug_service = use_project_slug_service()?;

    let use_case = GetProjectSlugsUseCase::new(project_slug_service);

    run_use_case(use_case, ()).await
}
