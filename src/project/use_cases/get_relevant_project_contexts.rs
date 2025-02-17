use crate::{
    core::{
        error::{
            named::internal_server_error::InternalServerError,
            server_function_error::ServerFunctionError,
        },
        use_case::UseCase,
    },
    project::{
        dto::project_contexts::ProjectContextsDto,
        services::project_context::ProjectContextService,
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
}

impl UseCase<(), ProjectContextsDto> for GetRelevantProjectContextsUseCase {
    async fn run(
        &mut self,
        _: (),
    ) -> Result<ProjectContextsDto, ServerFunctionError> {
        match self
            .project_context_service
            .get_relevant_project_contexts(3)
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
