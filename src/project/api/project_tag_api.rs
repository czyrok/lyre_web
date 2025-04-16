use leptos::{prelude::ServerFnError, server, server_fn::codec::GetUrl};

use crate::{
    core::error::server_error_dto::ServerErrorDto,
    project::dto::project_tags_dto::ProjectTagsDto,
};

#[server(prefix = "/api", endpoint = "project_tags", input = GetUrl)]
pub async fn get_all_project_tags(
) -> Result<ProjectTagsDto, ServerFnError<ServerErrorDto>> {
    use crate::{
        project::use_cases::get_all_project_tags_use_case::GetAllProjectTagsUseCase,
        system::{
            runner::use_case_runner::run_use_case,
            state::backend_contexts::use_project_tag_service,
        },
    };

    let project_tag_service = use_project_tag_service()?;

    let use_case = GetAllProjectTagsUseCase::new(project_tag_service);

    run_use_case(use_case, ()).await
}
