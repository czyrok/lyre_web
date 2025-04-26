use leptos::prelude::*;

use crate::{
    core::data::fetch_state::FetchState,
    project::resources::all_project_tags_resource::AllProjectTagsResource,
    shared::{
        button::{
            components::secondary_button::SecondaryButton,
            types::button_action::ButtonAction,
        },
        components::dropdown_menu::Position,
        enums::component_size::ComponentSize,
        select::{
            components::select::Select,
            types::{
                multi_select::MultiSelectChoices,
                select_choices_behavior::SelectChoicesBehavior,
            },
        },
    },
};

#[component]
pub fn TagSelector(
    set_selected_project_tags: WriteSignal<Vec<String>>,
    reset_event: Signal<()>,
) -> impl IntoView {
    let project_tag_resource = AllProjectTagsResource::default();

    let project_tags: RwSignal<Vec<String>> = RwSignal::new(vec![]);

    Effect::new(move |previous_value: Option<Vec<String>>| {
        let project_tags = project_tags.get();

        let mut needs_trigger_update = false;

        if let Some(previous_value) = previous_value {
            needs_trigger_update = previous_value != project_tags;
        }

        if needs_trigger_update {
            set_selected_project_tags.set(project_tags.clone());
        }

        project_tags
    });

    let get_tag_select_choices = async move || {
        let tag_choices = project_tag_resource.get_select_choices().await?;

        Ok(MultiSelectChoices::new(tag_choices, project_tags))
    };

    view! {
        <Suspense fallback=move || view! {
            <div class="tw-secondary-button-skeleton tw-button-size-md"></div>
        }>
            <ErrorBoundary fallback=|_| {
                let (is_errored, _) = signal(true);

                view! {
                    <SecondaryButton size=ComponentSize::MD text="Erreur" on_click=ButtonAction::None is_errored />
                }
            }>
                {move || Suspend::<Result<_, FetchState>>::new(async move {
                    get_tag_select_choices().await.map(|select_choices| {
                        let cloned_select_choices = select_choices.clone();

                        Effect::new(move |last_event: Option<()>| {
                            reset_event.track();

                            cloned_select_choices.change_all_status(false, None);

                            let is_first_event = last_event.is_none();

                            if !is_first_event {
                                cloned_select_choices.change_all_status(false, None);
                            }
                        });

                        view! {
                            <Select
                                size=ComponentSize::MD
                                dropdown_menu_position=Position::Bottom
                                text="#Tags"
                                identifier="tag-selector"
                                select_choices=select_choices
                                shows_ping_when_least_one_selected=true
                            />
                        }
                    })
                })}
            </ErrorBoundary>
        </Suspense>
    }
}
