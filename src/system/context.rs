use leptos::*;
use prelude::use_context;

use crate::common::error::{
    named::internal_server_error::InternalServerError,
    server_function_error::ServerFunctionException,
};
use crate::project::data::project_service::ProjectService;

pub fn use_project_service() -> Result<ProjectService<'static>, ServerFunctionException> {
    match use_context::<ProjectService<'static>>() {
        Some(project_service) => Ok(project_service),
        None => return Err(InternalServerError::new(Some("Context not found".into())).into()),
    }
}
