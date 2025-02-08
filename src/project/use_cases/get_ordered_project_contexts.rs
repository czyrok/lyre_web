use crate::{
    common::{
        cursor_pagination::CursorPagination,
        error::{
            named::{
                bad_request_server_error::BadRequestServerError,
                internal_server_error::InternalServerError,
            },
            server_function_error::ServerFunctionError,
        },
        use_case::UseCase,
    },
    project::{
        dto::project_contexts::ProjectContextsDto,
        services::{
            project::ProjectService, project_context::ProjectContextService,
        },
    },
};

pub struct GetOrderedProjectContextsUseCase {
    project_service: ProjectService,
    project_context_service: ProjectContextService,
}

impl GetOrderedProjectContextsUseCase {
    async fn check_slug_pagination_cursor_after(
        &self,
        pagination: &CursorPagination,
    ) -> Result<(), ServerFunctionError> {
        if let Some(slug_cursor_after) = pagination.cursor_after.clone() {
            let exists = match self
                .project_service
                .exists_project_from_slug(&slug_cursor_after)
                .await
            {
                Ok(exists) => exists,
                Err(error) => {
                    let internal_server_error =
                        InternalServerError::new_unable_to_check_if_project_exists(
                            format!(
                                "Unable to check if the project exists: `{}`",
                                error
                            ),
                        );

                    return Err(internal_server_error.into());
                }
            };

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

impl UseCase<CursorPagination, ProjectContextsDto>
    for GetOrderedProjectContextsUseCase
{
    async fn run(
        &mut self,
        pagination: CursorPagination,
    ) -> Result<ProjectContextsDto, ServerFunctionError> {
        self.check_slug_pagination_cursor_after(&pagination).await?;

        match self
            .project_context_service
            .get_ordered_project_contexts(
                pagination.limit,
                pagination.cursor_after.clone(),
            )
            .await
        {
            Ok(project_contexts) => {
                Ok(ProjectContextsDto::new(project_contexts))
            }
            Err(error) => {
                let internal_server_error =
                    InternalServerError::new_unable_to_get_project_contexts(
                        format!(
                            "Unable to get the project contexts: `{}`",
                            error
                        ),
                    );

                Err(internal_server_error.into())
            }
        }
    }
}
