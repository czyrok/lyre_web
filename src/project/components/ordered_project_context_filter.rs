use leptos::{prelude::*, web_sys::window};
use leptos_use::signal_debounced;

use crate::project::{
    components::{
        implementation_year_selector::ImplementationYearSelector,
        searched_project_title_input::SearchedProjectTitleInput,
        tag_selector::TagSelector,
    },
    dto::project_context_filter_dto::ProjectContextFilterDto,
};

#[component]
pub fn OrderedProjectContextFilter(
    default_filter: ProjectContextFilterDto,
    on_update: impl Fn(ProjectContextFilterDto) + 'static,
) -> impl IntoView {
    let (searched_project_title, set_searched_project_title) =
        signal("".into());

    let searched_project_title: Signal<String> =
        signal_debounced(searched_project_title, 1000.0);
    let (selected_implementation_years, set_selected_implementation_years) =
        signal(default_filter.implementation_years);
    let (selected_project_tags, set_selected_project_tags) =
        signal(default_filter.tags);

    let project_context_filter = Signal::derive(move || {
        let searched_project_title = searched_project_title.get();
        let searched_project_title = (!searched_project_title.is_empty())
            .then_some(searched_project_title);

        let selected_project_tags = selected_project_tags.get();
        let selected_implementation_years = selected_implementation_years.get();

        ProjectContextFilterDto::new(
            searched_project_title,
            selected_project_tags,
            selected_implementation_years,
        )
    });

    Effect::new(move |previous_value: Option<ProjectContextFilterDto>| {
        let project_context_filter = project_context_filter.get();

        let mut needs_trigger_update = false;

        if let Some(previous_value) = previous_value {
            needs_trigger_update = previous_value != project_context_filter;
        }

        if needs_trigger_update {
            on_update(project_context_filter.clone());
        }

        project_context_filter
    });

    let scroll_trigger = RwSignal::new(());

    Effect::new(move || {
        scroll_trigger.track();

        if let Some(window) = window() {
            let current_scroll = window.scroll_y().unwrap_or(0.0);
            let scroll_wanted = 300.0;

            if current_scroll < scroll_wanted {
                window.scroll_to_with_x_and_y(0.0, scroll_wanted);
            }
        }
    });

    view! {
        <div class="tw-flex tw-flex-col tw-flex-row tw-items-center tw-gap-level2">
            <SearchedProjectTitleInput set_searched_project_title scroll_trigger />

            <div class="tw-flex tw-flex-wrap tw-flex-row tw-gap-level1">
                <TagSelector set_selected_project_tags scroll_trigger />

                <ImplementationYearSelector set_selected_implementation_years scroll_trigger />
             </div>
        </div>
    }
}
