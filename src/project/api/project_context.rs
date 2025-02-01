use leptos::{prelude::ServerFnError, server, server_fn::codec::GetUrl};

use crate::{
    common::{
        cursor_pagination::CursorPagination,
        error::server_error_dto::ServerErrorDto,
    },
    project::dto::project_contexts::ProjectContextsDto,
};

#[server(prefix = "/api", endpoint = "project_contexts", input = GetUrl)]
pub async fn get_ordered_project_contexts(
    pagination: CursorPagination,
) -> Result<ProjectContextsDto, ServerFnError<ServerErrorDto>> {
    use crate::{
        project::use_cases::get_ordered_project_contexts::GetOrderedProjectContextsUseCase,
        system::{context::use_project_service, use_case_runner::run_use_case},
    };

    let project_service = use_project_service()?;

    let use_case = GetOrderedProjectContextsUseCase::new(project_service);

    let use_case_result = run_use_case(use_case, pagination).await;

    use_case_result
}

#[server(prefix = "/api", endpoint = "project_contexts", input = GetUrl)]
pub async fn get_relevant_project_contexts(
) -> Result<ProjectContextsDto, ServerFnError<ServerErrorDto>> {
    use crate::{
        project::use_cases::get_relevant_project_contexts::GetRelevantProjectContextsUseCase,
        system::{context::use_project_service, use_case_runner::run_use_case},
    };

    let project_service = use_project_service()?;

    let use_case = GetRelevantProjectContextsUseCase::new(project_service);

    let use_case_result = run_use_case(use_case, ()).await;

    use_case_result
}
