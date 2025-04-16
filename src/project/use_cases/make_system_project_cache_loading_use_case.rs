use std::fmt::Display;

use crate::{
    core::{
        behaviors::use_case::UseCase,
        error::{
            named::internal_server_error::InternalServerError,
            server_function_error::ServerFunctionError,
        },
    },
    project::services::project_service::ProjectService,
    system::{
        database::local_database::LocalDatabase,
        state::environment_context::EnvironmentContext,
    },
};

pub struct MakeSystemProjectCacheLoadingUseCase {
    environment: EnvironmentContext,
    project_service: ProjectService,
}

impl MakeSystemProjectCacheLoadingUseCase {
    pub fn new(
        environment: EnvironmentContext,
        project_service: ProjectService,
    ) -> Self {
        Self {
            environment,
            project_service,
        }
    }

    fn to_server_function_error<TError: Display>(
        error: TError,
    ) -> ServerFunctionError {
        let internal_server_error =
            InternalServerError::new_refresh_project_cache_failed(format!(
                "The project cache's loading failed: `{}`",
                error
            ));

        internal_server_error.into()
    }
}

impl UseCase<(), ()> for MakeSystemProjectCacheLoadingUseCase {
    async fn run(&mut self, _: ()) -> Result<(), ServerFunctionError> {
        let mut local_database = LocalDatabase::new(
            &self.environment.local_database_uri,
        )
        .await
        .map_err(
            MakeSystemProjectCacheLoadingUseCase::to_server_function_error,
        )?;

        let mut local_database_transaction =
            local_database.make_transaction().await.map_err(
                MakeSystemProjectCacheLoadingUseCase::to_server_function_error,
            )?;

        match self
            .project_service
            .refresh_project_cache(&mut local_database_transaction)
            .await
            .map_err(
                MakeSystemProjectCacheLoadingUseCase::to_server_function_error,
            ) {
            Ok(()) => (),
            Err(error) => {
                local_database_transaction.value.rollback().await.map_err(
                    MakeSystemProjectCacheLoadingUseCase::to_server_function_error,
                )?;

                return Err(error);
            }
        };

        local_database_transaction.value.commit().await.map_err(
            MakeSystemProjectCacheLoadingUseCase::to_server_function_error,
        )?;

        Ok(())
    }
}
