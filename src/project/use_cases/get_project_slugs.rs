use crate::{
    common::{
        error::{
            named::internal_server_error::InternalServerError,
            server_function_error::ServerFunctionError,
        },
        use_case::UseCase,
    },
    project::{
        dto::project_slugs::ProjectSlugsDto,
        services::project_slug::ProjectSlugService,
    },
};

pub struct GetProjectSlugsUseCase {
    project_slug_service: ProjectSlugService,
}

impl GetProjectSlugsUseCase {
    pub fn new(project_slug_service: ProjectSlugService) -> Self {
        Self {
            project_slug_service,
        }
    }
}

impl UseCase<(), ProjectSlugsDto> for GetProjectSlugsUseCase {
    async fn run(
        &mut self,
        _: (),
    ) -> Result<ProjectSlugsDto, ServerFunctionError> {
        match self.project_slug_service.get_project_slugs().await {
            Ok(project_slugs) => Ok(ProjectSlugsDto::new(project_slugs)),
            Err(error) => {
                let internal_server_error =
                    InternalServerError::new_unable_to_get_project_slugs(
                        format!("Unable to get the project slugs: `{}`", error),
                    );

                Err(internal_server_error.into())
            }
        }
    }
}
