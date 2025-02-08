use crate::{
    common::{
        error::{
            named::internal_server_error::InternalServerError,
            server_function_error::ServerFunctionError,
        },
        use_case::UseCase,
    },
    project::services::project::ProjectService,
};

pub struct RefreshProjectCacheUseCase {
    project_service: ProjectService,
}

impl RefreshProjectCacheUseCase {
    pub fn new(project_service: ProjectService) -> Self {
        Self { project_service }
    }
}

impl UseCase<(), ()> for RefreshProjectCacheUseCase {
    async fn run(&mut self, _: ()) -> Result<(), ServerFunctionError> {
        match self.project_service.refresh_project_cache().await {
            Ok(()) => Ok(()),
            Err(error) => {
                let internal_server_error =
                    InternalServerError::new_refresh_project_cache_failed(
                        format!(
                            "The refresh of the project cache failed: `{}`",
                            error
                        ),
                    );

                Err(internal_server_error.into())
            }
        }
    }
}
