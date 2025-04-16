use leptos::{prelude::ServerFnError, server, server_fn::codec::GetUrl};

use crate::{
    core::{
        dto::cursor_pagination_dto::CursorPaginationDto,
        error::server_error_dto::ServerErrorDto,
    },
    project::dto::{
        project_context_filter_dto::ProjectContextFilterDto,
        project_contexts_dto::ProjectContextsDto,
        relevant_project_contexts_dto::RelevantProjectContextsDto,
    },
};

#[server(prefix = "/api", endpoint = "project_contexts", input = GetUrl)]
pub async fn get_ordered_project_contexts(
    pagination: Option<CursorPaginationDto>,
    filter: Option<ProjectContextFilterDto>,
) -> Result<ProjectContextsDto, ServerFnError<ServerErrorDto>> {
    use crate::{
        project::use_cases::get_ordered_project_contexts_use_case::GetOrderedProjectContextsUseCase,
        system::{
            runner::use_case_runner::run_use_case,
            state::backend_contexts::{
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

    run_use_case(
        use_case,
        (pagination.unwrap_or_default(), filter.unwrap_or_default()),
    )
    .await
}

#[server(prefix = "/api", endpoint = "project_contexts/relevants", input = GetUrl)]
pub async fn get_relevant_project_contexts(
) -> Result<RelevantProjectContextsDto, ServerFnError<ServerErrorDto>> {
    use crate::{
        project::use_cases::get_relevant_project_contexts_use_case::GetRelevantProjectContextsUseCase,
        system::{
            runner::use_case_runner::run_use_case,
            state::backend_contexts::use_project_context_service,
        },
    };

    let project_context_service = use_project_context_service()?;

    let use_case =
        GetRelevantProjectContextsUseCase::new(project_context_service);

    run_use_case(use_case, ()).await
}
