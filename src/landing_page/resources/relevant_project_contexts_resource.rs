use leptos::prelude::*;

use crate::{
    core::data::fetch_state::FetchState,
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
    pub fn get_fetched_project_contexts(&self) -> Vec<ProjectContext> {
        self.0
            .get()
            .map(|n| n.unwrap_or_default())
            .map(|project_contexts_dto| project_contexts_dto.project_contexts)
            .unwrap_or_default()
    }

    pub fn get_fetch_state(&self) -> FetchState {
        let resource_result = self.0.get();

        if let Some(resource_result) = resource_result {
            let resource_result = resource_result.map(|_| ());

            if let Err(error) = resource_result {
                return FetchState::Errored(error);
            }

            return FetchState::Resolved;
        }

        FetchState::Pending
    }
}

impl Default for RelevantProjectContextsResource {
    fn default() -> Self {
        let resource = OnceResource::new(get_relevant_project_contexts());

        Self(resource)
    }
}
