use crate::common::error::{
    frontend_error_type::FrontedErrorType,
    server_error_dto::ServerErrorDto,
    server_function_error::{ServerFunctionError, ServerFunctionException},
};

pub struct BadRequestServerError {
    error_type: Option<FrontedErrorType>,
    detailed_message: Option<String>,
}

impl BadRequestServerError {
    pub fn new(detailed_message: Option<String>) -> Self {
        Self {
            error_type: None,
            detailed_message,
        }
    }

    pub fn new_unknown_project(detailed_message: String) -> Self {
        Self {
            error_type: Some(FrontedErrorType::unknown_project()),
            detailed_message: Some(detailed_message),
        }
    }
}

impl From<BadRequestServerError> for ServerErrorDto {
    fn from(server_error: BadRequestServerError) -> ServerErrorDto {
        let error_type = match server_error.error_type {
            Some(error_type) => error_type,
            None => FrontedErrorType::default(),
        };

        let detailed_message = match server_error.detailed_message {
            Some(message) => message,
            None => "Bad request".to_string(),
        };

        ServerErrorDto::new(400, error_type, detailed_message)
    }
}

impl From<BadRequestServerError> for ServerFunctionError {
    fn from(server_error: BadRequestServerError) -> ServerFunctionError {
        ServerFunctionError::WrappedServerError(server_error.into())
    }
}

impl From<BadRequestServerError> for ServerFunctionException {
    fn from(server_error: BadRequestServerError) -> ServerFunctionException {
        ServerFunctionException::WrappedServerError(server_error.into())
    }
}
