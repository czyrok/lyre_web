use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::frontend_error_type::FrontedErrorType;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ServerErrorDto {
    pub status_code: u16,
    pub error_type: FrontedErrorType,
    pub detailed_message: String,
}

impl ServerErrorDto {
    pub fn new(
        status_code: u16,
        error_type: FrontedErrorType,
        detailed_message: String,
    ) -> Self {
        Self {
            status_code,
            error_type,
            detailed_message,
        }
    }
}

impl std::fmt::Display for ServerErrorDto {
    fn fmt(
        &self,
        fmt: &mut std::fmt::Formatter<'_>,
    ) -> Result<(), std::fmt::Error> {
        let serialized_dto = serde_json::to_string(&self).unwrap();

        write!(fmt, "{}", serialized_dto)
    }
}

impl FromStr for ServerErrorDto {
    type Err = ();

    fn from_str(serialized_dto: &str) -> Result<Self, Self::Err> {
        let deserialized_dto: ServerErrorDto =
            serde_json::from_str(serialized_dto).unwrap();

        Ok(deserialized_dto)
    }
}
