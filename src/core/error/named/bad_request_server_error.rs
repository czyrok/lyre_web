use super::super::{
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
            error_type: Some(FrontedErrorType::UnknownProject),
            detailed_message: Some(detailed_message),
        }
    }

    pub fn new_invalid_totp_token(detailed_message: String) -> Self {
        Self {
            error_type: Some(FrontedErrorType::InvalidTotpToken),
            detailed_message: Some(detailed_message),
        }
    }
}

impl From<BadRequestServerError> for ServerErrorDto {
    fn from(server_error: BadRequestServerError) -> ServerErrorDto {
        let error_type = server_error.error_type.unwrap_or_default();

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
