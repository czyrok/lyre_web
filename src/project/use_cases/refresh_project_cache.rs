use std::error::Error;

use crate::{
    common::{
        error::{
            named::internal_server_error::InternalServerError,
            server_function_error::ServerFunctionError,
        },
        use_case::UseCase,
    },
    project::services::project::ProjectService,
    system::{
        database::local_database::LocalDatabase,
        environment_context::EnvironmentContext,
    },
};

pub struct RefreshProjectCacheUseCase {
    environment: EnvironmentContext,
    project_service: ProjectService,
}

impl RefreshProjectCacheUseCase {
    pub fn new(
        environment: EnvironmentContext,
        project_service: ProjectService,
    ) -> Self {
        Self {
            environment,
            project_service,
        }
    }

    fn sqlx_to_server_function_error(
        error: sqlx::Error,
    ) -> ServerFunctionError {
        let internal_server_error =
            InternalServerError::new_refresh_project_cache_failed(format!(
                "The refresh of the project cache failed: `{}`",
                error
            ));

        internal_server_error.into()
    }

    fn any_to_server_function_error(
        error: Box<dyn Error>,
    ) -> ServerFunctionError {
        let internal_server_error =
            InternalServerError::new_refresh_project_cache_failed(format!(
                "The refresh of the project cache failed: `{}`",
                error
            ));

        internal_server_error.into()
    }
}

impl UseCase<(), ()> for RefreshProjectCacheUseCase {
    async fn run(&mut self, _: ()) -> Result<(), ServerFunctionError> {
        let mut local_database =
            LocalDatabase::new(&self.environment.local_database_uri)
                .await
                .map_err(
                    RefreshProjectCacheUseCase::sqlx_to_server_function_error,
                )?;

        let mut local_database_transaction =
            local_database.make_transaction().await.map_err(
                RefreshProjectCacheUseCase::sqlx_to_server_function_error,
            )?;

        match self
            .project_service
            .refresh_project_cache(&mut local_database_transaction)
            .await
            .map_err(RefreshProjectCacheUseCase::any_to_server_function_error)
        {
            Ok(()) => (),
            Err(error) => {
                local_database_transaction.value.rollback().await.map_err(
                    RefreshProjectCacheUseCase::sqlx_to_server_function_error,
                )?;

                return Err(error);
            }
        };

        local_database_transaction.value.commit().await.map_err(
            RefreshProjectCacheUseCase::sqlx_to_server_function_error,
        )?;

        Ok(())
    }
}
