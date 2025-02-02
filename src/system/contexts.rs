use leptos::*;
use prelude::use_context;

use crate::{
    common::error::{
        named::internal_server_error::InternalServerError,
        server_function_error::ServerFunctionException,
    },
    project::data::project_service::ProjectService,
};

pub fn use_project_service() -> Result<ProjectService, ServerFunctionException>
{
    match use_context::<ProjectService>() {
        Some(project_service) => Ok(project_service),
        None => {
            Err(InternalServerError::new(Some("Context not found".into()))
                .into())
        }
    }
}
