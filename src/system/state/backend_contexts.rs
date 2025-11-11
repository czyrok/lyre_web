use leptos::*;
use prelude::use_context;

use super::environment_context::EnvironmentContext;
use crate::{
    core::error::{
        named::internal_server_error::InternalServerError,
        server_function_error::ServerFunctionError,
    },
    project::services::{
        project_context_service::ProjectContextService,
        project_service::ProjectService,
        project_slug_service::ProjectSlugService,
        project_tag_service::ProjectTagService,
    },
};

pub fn use_project_service() -> Result<ProjectService, ServerFunctionError> {
    match use_context::<ProjectService>() {
        Some(project_service) => Ok(project_service),
        None => Err(InternalServerError::new_context_not_found(
            "project_service".into(),
        )
        .into()),
    }
}

pub fn use_project_context_service(
) -> Result<ProjectContextService, ServerFunctionError> {
    match use_context::<ProjectContextService>() {
        Some(project_context_service) => Ok(project_context_service),
        None => Err(InternalServerError::new_context_not_found(
            "project_context_service".into(),
        )
        .into()),
    }
}

pub fn use_project_slug_service(
) -> Result<ProjectSlugService, ServerFunctionError> {
    match use_context::<ProjectSlugService>() {
        Some(project_slug_service) => Ok(project_slug_service),
        None => Err(InternalServerError::new_context_not_found(
            "project_slug_service".into(),
        )
        .into()),
    }
}

pub fn use_project_tag_service(
) -> Result<ProjectTagService, ServerFunctionError> {
    match use_context::<ProjectTagService>() {
        Some(project_tag_service) => Ok(project_tag_service),
        None => Err(InternalServerError::new_context_not_found(
            "project_tag_service".into(),
        )
        .into()),
    }
}

pub fn use_environment_context(
) -> Result<EnvironmentContext, ServerFunctionError> {
    match use_context::<EnvironmentContext>() {
        Some(environment) => Ok(environment),
        None => Err(InternalServerError::new_context_not_found(
            "environment_context".into(),
        )
        .into()),
    }
}
