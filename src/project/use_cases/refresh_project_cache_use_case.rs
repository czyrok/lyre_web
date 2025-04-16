use std::fmt::Display;

use crate::{
    core::{
        behaviors::use_case::UseCase,
        error::{
            named::{
                bad_request_server_error::BadRequestServerError,
                internal_server_error::InternalServerError,
            },
            server_function_error::ServerFunctionError,
        },
    },
    project::services::project_service::ProjectService,
    system::{
        database::local_database::LocalDatabase,
        state::environment_context::EnvironmentContext,
        totp::content_totp::ContentTotp,
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

    fn to_server_function_error<TError: Display>(
        error: TError,
    ) -> ServerFunctionError {
        let internal_server_error =
            InternalServerError::new_refresh_project_cache_failed(format!(
                "The refresh of the project cache failed: `{}`",
                error
            ));

        internal_server_error.into()
    }

    pub fn check_totp(&self, token: String) -> Result<(), ServerFunctionError> {
        let content_totp = ContentTotp::new(&self.environment.content_totp_uri)
            .map_err(RefreshProjectCacheUseCase::to_server_function_error)?;

        let is_token_valid = content_totp
            .check(&token)
            .map_err(RefreshProjectCacheUseCase::to_server_function_error)?;

        if !is_token_valid {
            let bad_request_error =
                BadRequestServerError::new_invalid_totp_token(format!(
                    "Your provided totp token `{}` is incorrect",
                    token
                ));

            return Err(bad_request_error.into());
        }

        Ok(())
    }

    pub async fn refresh_project_cache(
        &mut self,
    ) -> Result<(), ServerFunctionError> {
        let mut local_database =
            LocalDatabase::new(&self.environment.local_database_uri)
                .await
                .map_err(
                    RefreshProjectCacheUseCase::to_server_function_error,
                )?;

        let mut local_database_transaction = local_database
            .make_transaction()
            .await
            .map_err(RefreshProjectCacheUseCase::to_server_function_error)?;

        match self
            .project_service
            .refresh_project_cache(&mut local_database_transaction)
            .await
            .map_err(RefreshProjectCacheUseCase::to_server_function_error)
        {
            Ok(()) => (),
            Err(error) => {
                local_database_transaction.value.rollback().await.map_err(
                    RefreshProjectCacheUseCase::to_server_function_error,
                )?;

                return Err(error);
            }
        };

        local_database_transaction
            .value
            .commit()
            .await
            .map_err(RefreshProjectCacheUseCase::to_server_function_error)?;

        Ok(())
    }
}

impl UseCase<String, ()> for RefreshProjectCacheUseCase {
    async fn run(
        &mut self,
        totp_token: String,
    ) -> Result<(), ServerFunctionError> {
        self.check_totp(totp_token)?;

        self.refresh_project_cache().await?;

        Ok(())
    }
}
