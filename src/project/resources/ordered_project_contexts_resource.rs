use leptos::prelude::*;

use crate::{
    core::{
        data::fetch_state::FetchState,
        dto::cursor_pagination_dto::CursorPaginationDto,
    },
    project::{
        api::project_context_api::get_ordered_project_contexts,
        data::project_context::ProjectContext,
        dto::{
            project_context_filter_dto::ProjectContextFilterDto,
            project_contexts_dto::ProjectContextsDto,
        },
    },
};

#[derive(Clone, Copy)]
pub struct OrderedProjectContextsResource(
    Resource<
        Result<
            ProjectContextsDto,
            leptos::prelude::ServerFnError<
                crate::core::error::server_error_dto::ServerErrorDto,
            >,
        >,
    >,
);

impl OrderedProjectContextsResource {
    pub fn new(
        pagination: Signal<CursorPaginationDto>,
        filter: Signal<ProjectContextFilterDto>,
        set_is_loading: WriteSignal<bool>,
    ) -> Self {
        let resource = Resource::new(
            move || (pagination.get(), filter.get()),
            move |input| {
                set_is_loading.set(true);

                get_ordered_project_contexts(input.0.into(), input.1.into())
            },
        );

        Self(resource)
    }

    pub fn get_current_next_slug(&self) -> Option<String> {
        self.0
            .get()
            .map(|n| n.unwrap_or_default())
            .map(|project_contexts_dto| project_contexts_dto.project_contexts)
            .unwrap_or_default()
            .last()
            .map(|last_project_context| {
                last_project_context.next.clone().map(|next| next.slug)
            })
            .unwrap_or_default()
    }

    pub fn get_count_left(&self, current_project_count: usize) -> usize {
        self.0
            .get()
            .map(|n| n.unwrap_or_default())
            .map(|project_contexts_dto| {
                let total_count =
                    usize::try_from(project_contexts_dto.total_count)
                        .unwrap_or_default();

                let is_all_displayed = current_project_count >= total_count;

                if is_all_displayed {
                    return 0;
                }

                total_count.wrapping_sub(current_project_count)
            })
            .unwrap_or_default()
    }

    pub fn aggregate_fetched_project_contexts(
        &self,
        previous_project_contexts: Vec<ProjectContext>,
    ) -> Vec<ProjectContext> {
        let project_contexts = self
            .0
            .get()
            .map(|n| n.unwrap_or_default())
            .map(|project_contexts_dto| project_contexts_dto.project_contexts)
            .unwrap_or_default();

        let aggregate_project_contexts: Vec<ProjectContext> =
            previous_project_contexts
                .iter()
                .cloned()
                .chain(project_contexts.iter().cloned())
                .collect();

        aggregate_project_contexts
    }

    pub fn is_ready_untracked(&self) -> bool {
        let project_contexts =
            self.0.get_untracked().map(|n| n.unwrap_or_default());

        project_contexts.is_some()
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
