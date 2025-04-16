use leptos::prelude::*;

use crate::{
    core::dto::cursor_pagination_dto::CursorPaginationDto,
    project::{
        data::project_context::ProjectContext,
        resources::ordered_project_contexts_resource::OrderedProjectContextsResource,
    },
    shared::components::pagination::more_pagination::MorePagination,
};

#[component]
pub fn OrderedProjectContextPaginator(
    resource: OrderedProjectContextsResource,
    project_contexts: Signal<Vec<ProjectContext>>,
    set_pagination: WriteSignal<CursorPaginationDto>,
) -> impl IntoView {
    let (count_left, set_count_left): (ReadSignal<usize>, WriteSignal<usize>) =
        signal(0);
    let (current_next_slug, set_current_next_slug): (
        ReadSignal<Option<String>>,
        WriteSignal<Option<String>>,
    ) = signal(None);

    Effect::new(move |previous_value: Option<Option<String>>| {
        let current_next_slug = resource.get_current_next_slug();

        let needs_trigger_update;

        if let Some(previous_value) = previous_value {
            needs_trigger_update = previous_value != current_next_slug;
        } else {
            //// Needed to update once page is loaded
            needs_trigger_update = true;
        }

        if needs_trigger_update {
            set_current_next_slug.set(current_next_slug.clone());
        }

        current_next_slug
    });

    Effect::new(move |previous_value: Option<usize>| {
        let count_left = resource.get_count_left(project_contexts.get().len());

        let needs_trigger_update;

        if let Some(previous_value) = previous_value {
            needs_trigger_update = previous_value != count_left;
        } else {
            //// Needed to update once page is loaded
            needs_trigger_update = true;
        }

        if needs_trigger_update {
            set_count_left.set(count_left);
        }

        count_left
    });

    view! {
        <Show
            when=move || { count_left.get() > 0 }
        >
            <div class="tw-project-search-page-bottom-part">
                <MorePagination count_left=count_left.into() on_click=move |_| {
                    set_pagination.update(|pagination| pagination.cursor_after = current_next_slug.get_untracked())
                } />
            </div>
        </Show>
    }
}
