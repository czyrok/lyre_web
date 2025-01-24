use crate::{
    common::{
        cursor_pagination::CursorPagination,
        error::{
            named::bad_request_server_error::BadRequestServerError,
            server_function_error::ServerFunctionError,
        },
        use_case::UseCase,
    },
    project::{data::project_service::ProjectService, dto::project_contexts::ProjectContextsDto},
};

pub struct GetOrderedProjectContextsUseCase<'a> {
    project_service: ProjectService<'a>,
}

impl<'a> GetOrderedProjectContextsUseCase<'a> {
    fn check_slug_pagination_cursor_after(
        &self,
        pagination: &CursorPagination,
    ) -> Result<(), ServerFunctionError> {
        let test = match pagination.cursor_after.clone() {
            Some(slug_cursor_after) => self
                .project_service
                .exists_project_from_slug(&slug_cursor_after),
            None => true,
        };

        if test == false {
            let bad_request_error = BadRequestServerError::new_unknown_project(format!(
                "This project `{}` doesn't exist",
                pagination.cursor_after.clone().unwrap()
            ));

            return Err(bad_request_error.into());
        }

        Ok(())
    }

    pub fn new(project_service: ProjectService<'a>) -> Self {
        Self { project_service }
    }
}

impl<'a> UseCase<CursorPagination, ProjectContextsDto> for GetOrderedProjectContextsUseCase<'a> {
    async fn run(
        &self,
        pagination: CursorPagination,
    ) -> Result<ProjectContextsDto, ServerFunctionError> {
        self.check_slug_pagination_cursor_after(&pagination)?;

        let project_contexts = self
            .project_service
            .get_ordered_project_contexts(pagination.limit, pagination.cursor_after.clone());

        Ok(ProjectContextsDto::new(project_contexts))
    }
}
