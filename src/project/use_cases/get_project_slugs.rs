use crate::{
    common::{
        error::server_function_error::ServerFunctionError, use_case::UseCase,
    },
    project::{
        data::project_service::ProjectService,
        dto::project_slugs::ProjectSlugsDto,
    },
};

pub struct GetProjectSlugsUseCase<'a> {
    project_service: ProjectService<'a>,
}

impl<'a> GetProjectSlugsUseCase<'a> {
    pub fn new(project_service: ProjectService<'a>) -> Self {
        Self { project_service }
    }
}

impl<'a> UseCase<(), ProjectSlugsDto> for GetProjectSlugsUseCase<'a> {
    async fn run(&self, _: ()) -> Result<ProjectSlugsDto, ServerFunctionError> {
        let project_slugs = self.project_service.get_project_slugs();

        Ok(ProjectSlugsDto::new(project_slugs))
    }
}
