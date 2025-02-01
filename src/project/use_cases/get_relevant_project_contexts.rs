use crate::{
    common::{
        error::server_function_error::ServerFunctionError, use_case::UseCase,
    },
    project::{
        data::project_service::ProjectService,
        dto::project_contexts::ProjectContextsDto,
    },
};

pub struct GetRelevantProjectContextsUseCase {
    project_service: ProjectService,
}

impl GetRelevantProjectContextsUseCase {
    pub fn new(project_service: ProjectService) -> Self {
        Self { project_service }
    }
}

impl UseCase<(), ProjectContextsDto> for GetRelevantProjectContextsUseCase {
    async fn run(
        &self,
        _: (),
    ) -> Result<ProjectContextsDto, ServerFunctionError> {
        let project_contexts =
            self.project_service.get_relevant_project_contexts(3);

        Ok(ProjectContextsDto::new(project_contexts))
    }
}
