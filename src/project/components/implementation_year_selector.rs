use leptos::prelude::*;

use crate::{
    core::data::icon_set::IconSet,
    project::enums::implementation_year::ImplementationYear,
    shared::{
        components::dropdown_menu::Position,
        enums::component_size::ComponentSize,
        select::{
            components::select::Select,
            types::{
                multi_select::MultiSelectChoices, select_choice::SelectChoice,
                select_choices_behavior::SelectChoicesBehavior,
            },
        },
    },
};

#[component]
pub fn ImplementationYearSelector(
    set_selected_implementation_years: WriteSignal<Vec<ImplementationYear>>,
    reset_event: Signal<()>,
) -> impl IntoView {
    let implementation_years = RwSignal::new(vec![]);

    let select_choices = MultiSelectChoices::new(
        [
            SelectChoice::new(
                ImplementationYear::CurrentYear,
                "Cette année".into(),
                None,
            ),
            SelectChoice::new(
                ImplementationYear::LastYear,
                "L'année dernière".into(),
                None,
            ),
            SelectChoice::new(
                ImplementationYear::TwoYearsAgo,
                "Il y a 2 ans".into(),
                None,
            ),
            SelectChoice::new(
                ImplementationYear::MoreThanTwoYears,
                "Il y a plus de 2 ans".into(),
                None,
            ),
        ]
        .into(),
        implementation_years,
    );

    Effect::new(move |previous_value: Option<Vec<ImplementationYear>>| {
        let implementation_years = implementation_years.get();

        let mut needs_trigger_update = false;

        if let Some(previous_value) = previous_value {
            needs_trigger_update = previous_value != implementation_years;
        }

        if needs_trigger_update {
            set_selected_implementation_years.set(implementation_years.clone());
        }

        implementation_years
    });

    let cloned_select_choices = select_choices.clone();

    Effect::new(move |last_event: Option<()>| {
        reset_event.track();

        let is_first_event = last_event.is_none();

        if !is_first_event {
            cloned_select_choices.change_all_status(false, None);
        }
    });

    view! {
        <Select
            size=ComponentSize::MD
            icon=IconSet::Calendar
            dropdown_menu_position=Position::Bottom
            text="Réalisation"
            identifier="year-of-implementation-selector"
            select_choices=select_choices
            shows_ping_when_least_one_selected=true
        />
    }
}
