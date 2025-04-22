use core::fmt;
use std::fmt::{Display, Formatter};

use leptos::prelude::{Error, ServerFnError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::core::error::{
    server_error_dto::ServerErrorDto,
    server_function_error::ServerFunctionException,
};

#[derive(Default, Serialize, Deserialize, Error, Clone, Debug)]
pub enum FetchState {
    #[default]
    Pending,
    Resolved,
    Errored(ServerFunctionException),
}

impl FetchState {
    pub fn is_ok(&self) -> bool {
        matches!(self, Self::Pending | Self::Resolved)
    }

    pub fn is_errored(&self) -> bool {
        if let Self::Errored(_) = self {
            return true;
        }

        false
    }

    pub fn unwrap_error(self) -> ServerFunctionException {
        if let FetchState::Errored(error) = self {
            return error;
        }

        panic!("Not in a errored state")
    }

    pub fn get_error_dto_from(error: Error) -> Option<ServerErrorDto> {
        let error = error.into_inner();

        if let Some(FetchState::Errored(ServerFnError::WrappedServerError(
            server_error_dto,
        ))) = error.downcast_ref::<FetchState>()
        {
            return server_error_dto.clone().into();
        }

        None
    }
}

impl Display for FetchState {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "{:?}", self)
    }
}
