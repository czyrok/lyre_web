use crate::{
    common::{
        error::server_function_error::ServerFunctionError, use_case::UseCase,
    },
    project::{
        data::project_service::ProjectService,
        dto::project_slugs::ProjectSlugsDto,
    },
};

pub struct GetProjectSlugsUseCase {
    project_service: ProjectService,
}

impl GetProjectSlugsUseCase {
    pub fn new(project_service: ProjectService) -> Self {
        Self { project_service }
    }
}

impl UseCase<(), ProjectSlugsDto> for GetProjectSlugsUseCase {
    async fn run(&self, _: ()) -> Result<ProjectSlugsDto, ServerFunctionError> {
        let project_slugs = self.project_service.get_project_slugs();

        Ok(ProjectSlugsDto::new(project_slugs))
    }
}
