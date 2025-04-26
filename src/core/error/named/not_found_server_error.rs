use super::super::{
    frontend_error_type::FrontedErrorType,
    server_error_dto::ServerErrorDto,
    server_function_error::{ServerFunctionError, ServerFunctionException},
};

pub struct NotFoundServerError {
    error_type: Option<FrontedErrorType>,
    detailed_message: Option<String>,
}

impl NotFoundServerError {
    pub fn new(detailed_message: Option<String>) -> Self {
        Self {
            error_type: None,
            detailed_message,
        }
    }

    pub fn new_project_not_found(detailed_message: String) -> Self {
        Self {
            error_type: Some(FrontedErrorType::ProjectNotFound),
            detailed_message: Some(detailed_message),
        }
    }
}

impl From<NotFoundServerError> for ServerErrorDto {
    fn from(server_error: NotFoundServerError) -> ServerErrorDto {
        let error_type = server_error.error_type.unwrap_or_default();

        let detailed_message = match server_error.detailed_message {
            Some(message) => message,
            None => "Not found".to_string(),
        };

        ServerErrorDto::new(404, error_type, detailed_message)
    }
}

impl From<NotFoundServerError> for ServerFunctionError {
    fn from(server_error: NotFoundServerError) -> ServerFunctionError {
        ServerFunctionError::WrappedServerError(server_error.into())
    }
}

impl From<NotFoundServerError> for ServerFunctionException {
    fn from(server_error: NotFoundServerError) -> ServerFunctionException {
        ServerFunctionException::WrappedServerError(server_error.into())
    }
}
