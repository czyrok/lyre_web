use leptos::prelude::*;
use leptos_use::signal_debounced;

use crate::{
    core::data::icon_set::IconSet,
    shared::{
        components::input_text::InputText, enums::component_size::ComponentSize,
    },
};

#[component]
pub fn SearchedProjectTitleInput(
    set_searched_project_title: WriteSignal<String>,
    reset_event: Signal<()>,
) -> impl IntoView {
    let (self_searched_project_title, set_self_searched_project_title) =
        signal("".into());

    let delayed_searched_project_title: Signal<String> =
        signal_debounced(self_searched_project_title, 1500.0);

    Effect::new(move |previous_value: Option<String>| {
        let new_search_project_title = delayed_searched_project_title.get();

        let mut needs_update = false;

        if let Some(previous_value) = previous_value {
            needs_update = previous_value != new_search_project_title;
        }

        if needs_update {
            set_searched_project_title.set(new_search_project_title.clone());
        }

        new_search_project_title
    });

    Effect::new(move |last_event: Option<()>| {
        reset_event.track();

        let is_first_event = last_event.is_none();

        if !is_first_event {
            set_searched_project_title.set("".into());
            set_self_searched_project_title.set("".into());
        }
    });

    view! {
        <InputText
            size=ComponentSize::LG
            text=(self_searched_project_title, set_self_searched_project_title)
            placeholder="Nom d'un projet"
            icon=IconSet::Search
            additional_style_classes="tw-searched-project-title-input-text"
        />
    }
}
