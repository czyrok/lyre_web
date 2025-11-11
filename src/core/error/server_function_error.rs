use leptos::{
    prelude::FromServerFnError,
    server_fn::{codec::JsonEncoding, error::ServerFnErrorErr},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::server_error_dto::ServerErrorDto;
use crate::core::error::named::internal_server_error::InternalServerError;

#[derive(Debug, Clone, Error, Deserialize, Serialize)]
pub struct ServerFunctionError(pub ServerErrorDto);

impl std::fmt::Display for ServerFunctionError {
    fn fmt(
        &self,
        fmt: &mut std::fmt::Formatter<'_>,
    ) -> Result<(), std::fmt::Error> {
        let serialized_error = self.0.to_string();

        write!(fmt, "{serialized_error}")
    }
}

impl FromServerFnError for ServerFunctionError {
    type Encoder = JsonEncoding;

    fn from_server_fn_error(value: ServerFnErrorErr) -> Self {
        let server_error: InternalServerError = value.into();

        server_error.into()
    }
}

impl From<ServerErrorDto> for ServerFunctionError {
    fn from(server_error_dto: ServerErrorDto) -> Self {
        Self(server_error_dto)
    }
}
