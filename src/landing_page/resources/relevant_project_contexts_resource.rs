use leptos::prelude::*;

use crate::{
    core::error::server_function_error::ServerFunctionException,
    project::{
        api::project_context_api::get_relevant_project_contexts,
        data::project_context::ProjectContext,
        dto::relevant_project_contexts_dto::RelevantProjectContextsDto,
    },
};

#[derive(Clone, Copy)]
pub struct RelevantProjectContextsResource(
    OnceResource<
        Result<
            RelevantProjectContextsDto,
            leptos::prelude::ServerFnError<
                crate::core::error::server_error_dto::ServerErrorDto,
            >,
        >,
    >,
);

impl RelevantProjectContextsResource {
    // TODO: default impl
    pub fn new() -> Self {
        let resource = OnceResource::new(get_relevant_project_contexts());

        Self(resource)
    }

    pub fn get_fetched_project_contexts(&self) -> Vec<ProjectContext> {
        self.0
            .get()
            .map(|n| n.unwrap_or_default())
            .map(|project_contexts_dto| project_contexts_dto.project_contexts)
            .unwrap_or_default()
    }

    pub fn track(&self) {
        self.0.track();
    }

    pub fn is_errored(&self) -> Result<(), ServerFunctionException> {
        let resource_result = self.0.get_untracked();

        if let Some(resource_result) = resource_result {
            let resource_result = resource_result.map(|_| ());

            return resource_result;
        }

        Ok(())
    }
}
