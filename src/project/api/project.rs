use leptos::{
    prelude::ServerFnError,
    server,
    server_fn::codec::{GetUrl, PostUrl},
};

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
        system::{
            contexts::use_project_service, use_case_runner::run_use_case,
        },
    };

    let project_service = use_project_service()?;

    let use_case = GetProjectUseCase::new(project_service);

    run_use_case(use_case, slug).await
}

#[server(prefix = "/api", endpoint = "projects/refresh", input = PostUrl)]
pub async fn refresh_project_cache() -> Result<(), ServerFnError<ServerErrorDto>>
{
    use crate::{
        project::use_cases::refresh_project_cache::RefreshProjectCacheUseCase,
        system::{
            contexts::{use_environment_context, use_project_service},
            use_case_runner::run_use_case,
        },
    };

    // TODO: recevoir un code OTP ? et le vérifier dans le use case
    // TODO: si on spam ça fait quoi ?

    let environment = use_environment_context()?;
    let project_service = use_project_service()?;

    let use_case =
        RefreshProjectCacheUseCase::new(environment, project_service);

    // TODO: vérifier si ca renvoit un 204

    run_use_case(use_case, ()).await
}
