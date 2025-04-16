use leptos::prelude::*;

use crate::{
    core::cursor_pagination::CursorPagination,
    project::{
        components::{
            empty_search_result::EmptySearchResult,
            ordered_project_context_filter::OrderedProjectContextFilter,
            ordered_project_context_paginator::OrderedProjectContextPaginator,
            project_card::ProjectCard,
            project_card_skeleton::ProjectCardSkeleton,
            searched_project_title_input::SearchedProjectTitleInput,
        },
        data::project_context::ProjectContext,
        dto::project_context_filter_dto::ProjectContextFilterDto,
        resources::ordered_project_contexts_resource::OrderedProjectContextsResource,
    },
    shared::layouts::secondary_page_layout::SecondaryPageLayout,
};

#[component]
pub fn ProjectSearchPage() -> impl IntoView {
    let (project_contexts, set_project_contexts): (
        ReadSignal<Vec<ProjectContext>>,
        WriteSignal<Vec<ProjectContext>>,
    ) = signal(vec![]);

    let (pagination, set_pagination) = signal(CursorPagination::default());
    let (project_context_filter, set_project_context_filter) =
        signal(ProjectContextFilterDto::default());
    let (searched_project_title, set_searched_project_title) =
        signal("".into());
    let (filter_reset_event, set_filter_reset_event) = signal(());

    let project_context_resource = OrderedProjectContextsResource::new(
        pagination.into(),
        project_context_filter.into(),
    );

    Effect::new(move || {
        let displayed_project_contexts = project_contexts.get_untracked();

        let aggregated_project_contexts = project_context_resource
            .aggregate_project_contexts(displayed_project_contexts);

        set_project_contexts.set(aggregated_project_contexts.clone());
    });

    let reset_view_when_filter_updated = move || {
        set_project_contexts.set(vec![]);
        set_pagination.set(CursorPagination::default());
    };

    view! {
        <SecondaryPageLayout
            content_render=move || {
                view! {
                    <div class="tw-project-search-page-top-part">
                        <h1 class="tw-title-size-lg">"Mes Projets"</h1>

                        <OrderedProjectContextFilter default_filter=project_context_filter.get_untracked() searched_project_title=searched_project_title.into() reset_event=(filter_reset_event, set_filter_reset_event) on_update=move |project_context_filter| {
                            set_project_context_filter.set(project_context_filter);
                            reset_view_when_filter_updated();
                        } />
                    </div>

                    <div class="tw-project-search-page-middle-part">
                        <SearchedProjectTitleInput set_searched_project_title reset_event=filter_reset_event.into() />

                        <div class="tw-middle-part-list">
                            <Suspense fallback=move || {
                                view! {
                                    <ProjectCardSkeleton />
                                }
                            }>
                                <For each=move || project_contexts.get() key=|project_context| project_context.slug.clone() let:project_context>
                                    <ProjectCard project_context=project_context />
                                </For>
                            </Suspense>
                        </div>
                    </div>

                    <EmptySearchResult project_contexts=project_contexts.into() />
                    <OrderedProjectContextPaginator resource=project_context_resource project_contexts=project_contexts.into() set_pagination />
                }
            }
        />
    }
}
