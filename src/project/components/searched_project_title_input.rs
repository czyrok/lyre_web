use leptos::prelude::*;
use leptos_use::signal_debounced;

use crate::{
    core::{data::icon_set::IconSet, types::closure::OnClickCallback},
    shared::{
        enums::component_size::ComponentSize,
        input_text::components::primary_input_text::PrimaryInputText,
    },
};

#[component]
pub fn SearchedProjectTitleInput(
    set_searched_project_title: WriteSignal<String>,
) -> impl IntoView {
    let (self_searched_project_title, set_self_searched_project_title) =
        signal("".into());

    let delayed_searched_project_title: Signal<String> =
        signal_debounced(self_searched_project_title, 100.0);

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

    let reset_callback: Box<dyn OnClickCallback> = Box::new(move |_| {
        set_searched_project_title.set("".into());
        set_self_searched_project_title.set("".into());
    });

    view! {
        <PrimaryInputText
            size=ComponentSize::LG
            text=(self_searched_project_title, set_self_searched_project_title)
            placeholder="Nom d'un projet, d'une techno."
            icon=IconSet::Search
            additional_style_classes="tw-max-w-48 tw-w-full"
            shows_active_state_when_has_text=true
            reset_callback
        />
    }
}
