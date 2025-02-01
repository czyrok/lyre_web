use crate::{
    common::{
        error::{
            named::not_found_server_error::NotFoundServerError,
            server_function_error::ServerFunctionError,
        },
        use_case::UseCase,
    },
    project::{
        data::project_service::ProjectService, dto::project::ProjectDto,
    },
};

pub struct GetProjectUseCase<'a> {
    project_service: ProjectService<'a>,
}

impl<'a> GetProjectUseCase<'a> {
    pub fn new(project_service: ProjectService<'a>) -> Self {
        Self { project_service }
    }
}

impl<'a> UseCase<String, ProjectDto> for GetProjectUseCase<'a> {
    async fn run(
        &self,
        slug: String,
    ) -> Result<ProjectDto, ServerFunctionError> {
        let project = match self.project_service.get_project(&slug) {
            Some(project) => project,
            None => {
                let not_found_error =
                    NotFoundServerError::new_project_not_found(format!(
                        "This project `{}` doesn't exist",
                        slug
                    ));

                return Err(not_found_error.into());
            }
        };

        Ok(ProjectDto::new(project))
    }
}
