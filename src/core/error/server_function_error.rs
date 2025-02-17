use leptos::{prelude::ServerFnError, server_fn::error::ServerFnErrorErr};

use super::server_error_dto::ServerErrorDto;

pub type ServerFunctionError = ServerFnErrorErr<ServerErrorDto>;
pub type ServerFunctionException = ServerFnError<ServerErrorDto>;
