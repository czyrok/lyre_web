use crate::{
    core::{
        behaviors::use_case::UseCase,
        error::{
            named::{
                internal_server_error::InternalServerError,
                not_found_server_error::NotFoundServerError,
            },
            server_function_error::ServerFunctionError,
        },
    },
    project::{
        dto::project_dto::ProjectDto, services::project_service::ProjectService,
    },
};

pub struct GetProjectUseCase {
    project_service: ProjectService,
}

impl GetProjectUseCase {
    pub fn new(project_service: ProjectService) -> Self {
        Self { project_service }
    }
}

impl UseCase<String, ProjectDto> for GetProjectUseCase {
    async fn run(
        &mut self,
        slug: String,
    ) -> Result<ProjectDto, ServerFunctionError> {
        let project = match self.project_service.get_project(&slug).await {
            Ok(project) => project,
            Err(error) => {
                let internal_server_error =
                    InternalServerError::new_unable_to_get_project(format!(
                        "Unable to get the project: `{}`",
                        error
                    ));

                return Err(internal_server_error.into());
            }
        };

        if let Some(project) = project {
            return Ok(ProjectDto::new(project));
        }

        let not_found_error = NotFoundServerError::new_project_not_found(
            format!("This project `{}` doesn't exist", slug),
        );

        Err(not_found_error.into())
    }
}
