use std::fmt::Display;

use crate::{
    core::{
        behaviors::use_case::UseCase,
        dto::cursor_pagination_dto::CursorPaginationDto,
        error::{
            named::{
                bad_request_server_error::BadRequestServerError,
                internal_server_error::InternalServerError,
            },
            server_function_error::ServerFunctionError,
        },
    },
    project::{
        dto::{
            project_context_filter_dto::ProjectContextFilterDto,
            project_contexts_dto::ProjectContextsDto,
        },
        services::{
            project_context_service::ProjectContextService,
            project_service::ProjectService,
        },
    },
};

pub struct GetOrderedProjectContextsUseCase {
    project_service: ProjectService,
    project_context_service: ProjectContextService,
}

impl GetOrderedProjectContextsUseCase {
    fn to_server_function_error<TError: Display>(
        error: TError,
    ) -> ServerFunctionError {
        let internal_server_error =
            InternalServerError::new_unable_to_get_project_contexts(format!(
                "Unable to get the project contexts: `{}`",
                error
            ));

        internal_server_error.into()
    }

    async fn check_slug_pagination_cursor_after(
        &self,
        pagination: &CursorPaginationDto,
    ) -> Result<(), ServerFunctionError> {
        if let Some(slug_cursor_after) = pagination.cursor_after.clone() {
            let exists = self
                .project_service
                .exists_project_from_slug(&slug_cursor_after)
                .await
                .map_err(
                    GetOrderedProjectContextsUseCase::to_server_function_error,
                )?;

            if !exists {
                let bad_request_error =
                    BadRequestServerError::new_unknown_project(format!(
                        "This project `{}` doesn't exist",
                        pagination.cursor_after.clone().unwrap()
                    ));

                return Err(bad_request_error.into());
            }
        }

        Ok(())
    }

    pub fn new(
        project_service: ProjectService,
        project_context_service: ProjectContextService,
    ) -> Self {
        Self {
            project_service,
            project_context_service,
        }
    }
}

impl UseCase<(CursorPaginationDto, ProjectContextFilterDto), ProjectContextsDto>
    for GetOrderedProjectContextsUseCase
{
    async fn run(
        &mut self,
        input: (CursorPaginationDto, ProjectContextFilterDto),
    ) -> Result<ProjectContextsDto, ServerFunctionError> {
        self.check_slug_pagination_cursor_after(&input.0).await?;

        let project_contexts = self
            .project_context_service
            .get_ordered_project_contexts(input.0, input.1)
            .await
            .map_err(
                GetOrderedProjectContextsUseCase::to_server_function_error,
            )?;

        Ok(ProjectContextsDto::new(
            project_contexts.0,
            project_contexts.1,
        ))
    }
}
