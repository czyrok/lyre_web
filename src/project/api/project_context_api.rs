use leptos::{prelude::ServerFnError, server, server_fn::codec::GetUrl};

use crate::{
    core::{
        cursor_pagination::CursorPagination,
        error::server_error_dto::ServerErrorDto,
    },
    project::dto::project_contexts_dto::ProjectContextsDto,
};

#[server(prefix = "/api", endpoint = "project_contexts", input = GetUrl)]
pub async fn get_ordered_project_contexts(
    pagination: CursorPagination,
) -> Result<ProjectContextsDto, ServerFnError<ServerErrorDto>> {
    use crate::{
        project::use_cases::get_ordered_project_contexts_use_case::GetOrderedProjectContextsUseCase,
        system::{
            runner::use_case_runner::run_use_case,
            state::contexts::{
                use_project_context_service, use_project_service,
            },
        },
    };

    let project_service = use_project_service()?;
    let project_context_service = use_project_context_service()?;

    let use_case = GetOrderedProjectContextsUseCase::new(
        project_service,
        project_context_service,
    );

    run_use_case(use_case, pagination).await
}

#[server(prefix = "/api", endpoint = "project_contexts/relevants", input = GetUrl)]
pub async fn get_relevant_project_contexts(
) -> Result<ProjectContextsDto, ServerFnError<ServerErrorDto>> {
    use crate::{
        project::use_cases::get_relevant_project_contexts_use_case::GetRelevantProjectContextsUseCase,
        system::{
            runner::use_case_runner::run_use_case,
            state::contexts::use_project_context_service,
        },
    };

    let project_context_service = use_project_context_service()?;

    let use_case =
        GetRelevantProjectContextsUseCase::new(project_context_service);

    run_use_case(use_case, ()).await
}
