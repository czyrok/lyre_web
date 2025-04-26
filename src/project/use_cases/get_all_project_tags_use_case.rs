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
        dto::project_tags_dto::ProjectTagsDto,
        services::project_tag_service::ProjectTagService,
    },
};

pub struct GetAllProjectTagsUseCase {
    project_tag_service: ProjectTagService,
}

impl GetAllProjectTagsUseCase {
    fn to_server_function_error<TError: Display>(
        error: TError,
    ) -> ServerFunctionError {
        let internal_server_error =
            InternalServerError::new_unable_to_get_project_tags(format!(
                "Unable to get the project tags: `{}`",
                error
            ));

        internal_server_error.into()
    }

    pub fn new(project_tag_service: ProjectTagService) -> Self {
        Self {
            project_tag_service,
        }
    }
}

impl UseCase<(), ProjectTagsDto> for GetAllProjectTagsUseCase {
    async fn run(
        &mut self,
        _: (),
    ) -> Result<ProjectTagsDto, ServerFunctionError> {
        let project_tags = self
            .project_tag_service
            .get_all_project_tags()
            .await
            .map_err(GetAllProjectTagsUseCase::to_server_function_error)?;

        Ok(ProjectTagsDto::new(project_tags))
    }
}
