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
    pub async fn get_project_contexts(
        &self,
    ) -> Result<Vec<ProjectContext>, FetchState> {
        self.0
            .await
            .map(|project_contexts_dto| project_contexts_dto.project_contexts)
            .map_err(FetchState::Errored)
    }
}

impl Default for RelevantProjectContextsResource {
    fn default() -> Self {
        let resource = OnceResource::new(get_relevant_project_contexts());

        Self(resource)
    }
}
