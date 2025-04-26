use leptos::prelude::*;
use leptos_use::signal_debounced;

use crate::{
    core::data::icon_set::IconSet,
    project::{
        components::{
            implementation_year_selector::ImplementationYearSelector,
            tag_selector::TagSelector,
        },
        dto::project_context_filter_dto::ProjectContextFilterDto,
    },
    shared::{
        button::components::secondary_button::SecondaryButton,
        enums::component_size::ComponentSize,
    },
};

#[component]
pub fn OrderedProjectContextFilter(
    default_filter: ProjectContextFilterDto,
    searched_project_title: Signal<String>,
    #[prop(name = "reset_event")] (reset_event, set_reset_event): (
        ReadSignal<()>,
        WriteSignal<()>,
    ),
    on_update: impl Fn(ProjectContextFilterDto) + 'static,
) -> impl IntoView {
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

    let reset_filters = move |_| {
        set_reset_event.set(());
    };

    view! {
        <div class="tw-top-part-filter">
            <ImplementationYearSelector set_selected_implementation_years reset_event=reset_event.into() />

            <TagSelector set_selected_project_tags reset_event=reset_event.into() />

            <SecondaryButton size=ComponentSize::MD icon=IconSet::Undo on_click=reset_filters />
        </div>
    }
}
