use super::super::{
    frontend_error_type::FrontedErrorType,
    server_error_dto::ServerErrorDto,
    server_function_error::{ServerFunctionError, ServerFunctionException},
};

pub struct InternalServerError {
    error_type: Option<FrontedErrorType>,
    detailed_message: Option<String>,
}

impl InternalServerError {
    pub fn new(detailed_message: Option<String>) -> Self {
        Self {
            error_type: None,
            detailed_message,
        }
    }

    pub fn new_refresh_project_cache_failed(detailed_message: String) -> Self {
        Self {
            error_type: Some(FrontedErrorType::RefreshProjectCacheFailed),
            detailed_message: Some(detailed_message),
        }
    }

    pub fn new_unable_to_get_project_slugs(detailed_message: String) -> Self {
        Self {
            error_type: Some(FrontedErrorType::UnableToGetProjectSlugs),
            detailed_message: Some(detailed_message),
        }
    }

    pub fn new_unable_to_get_project_contexts(
        detailed_message: String,
    ) -> Self {
        Self {
            error_type: Some(FrontedErrorType::UnableToGetProjectContexts),
            detailed_message: Some(detailed_message),
        }
    }

    pub fn new_unable_to_get_project(detailed_message: String) -> Self {
        Self {
            error_type: Some(FrontedErrorType::UnableToGetProject),
            detailed_message: Some(detailed_message),
        }
    }

    pub fn new_unable_to_get_project_tags(detailed_message: String) -> Self {
        Self {
            error_type: Some(FrontedErrorType::UnableToGetProjectTags),
            detailed_message: Some(detailed_message),
        }
    }
}

impl From<InternalServerError> for ServerErrorDto {
    fn from(server_error: InternalServerError) -> ServerErrorDto {
        let error_type = server_error.error_type.unwrap_or_default();

        let detailed_message = match server_error.detailed_message {
            Some(message) => message,
            None => "Internal server Error".to_string(),
        };

        ServerErrorDto::new(500, error_type, detailed_message)
    }
}

impl From<InternalServerError> for ServerFunctionError {
    fn from(server_error: InternalServerError) -> ServerFunctionError {
        ServerFunctionError::WrappedServerError(server_error.into())
    }
}

impl From<InternalServerError> for ServerFunctionException {
    fn from(server_error: InternalServerError) -> ServerFunctionException {
        ServerFunctionException::WrappedServerError(server_error.into())
    }
}
