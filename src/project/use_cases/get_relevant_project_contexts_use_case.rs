use std::fmt::Display;

use crate::{
    core::{
        behaviors::use_case::UseCase,
        error::{
            named::internal_server_error::InternalServerError,
            server_function_error::ServerFunctionError,
        },
    },
    project::{
        dto::relevant_project_contexts_dto::RelevantProjectContextsDto,
        services::project_context_service::ProjectContextService,
    },
};

pub struct GetRelevantProjectContextsUseCase {
    project_context_service: ProjectContextService,
}

impl GetRelevantProjectContextsUseCase {
    pub fn new(project_context_service: ProjectContextService) -> Self {
        Self {
            project_context_service,
        }
    }

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
}

impl UseCase<(), RelevantProjectContextsDto>
    for GetRelevantProjectContextsUseCase
{
    async fn run(
        &mut self,
        _: (),
    ) -> Result<RelevantProjectContextsDto, ServerFunctionError> {
        let project_contexts = self
            .project_context_service
            .get_relevant_project_contexts(3)
            .await
            .map_err(
                GetRelevantProjectContextsUseCase::to_server_function_error,
            )?;

        Ok(RelevantProjectContextsDto::new(project_contexts))
    }
}
