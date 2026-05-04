use leptos::prelude::*;

use crate::{
    core::{
        data::{fetch_state::FetchState, icon_set::IconSet},
        types::closure::OnClickCallback,
    },
    project::resources::all_project_tags_resource::AllProjectTagsResource,
    shared::{
        button::{
            components::primary_button::PrimaryButton,
            types::button_action::ButtonAction,
        },
        components::dropdown_menu::Position,
        enums::component_size::ComponentSize,
        select::{
            components::secondary_select::SecondarySelect,
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
                    <PrimaryButton size=ComponentSize::LG text="Erreur" on_click=ButtonAction::None is_errored />
                }
            }>
                {move || Suspend::<Result<_, FetchState>>::new(async move {
                    get_tag_select_choices().await.map(|select_choices| {
                        let cloned_select_choices = select_choices.clone();
                        let reset_callback: Box<dyn OnClickCallback> = Box::new(move |_| {
                            cloned_select_choices.change_all_status(false, None);
                        });

                        view! {
                            <SecondarySelect
                                size=ComponentSize::LG
                                dropdown_menu_position=Position::Bottom
                                icon=IconSet::Hashtag
                                text="Tags"
                                identifier="tag-selector"
                                select_choices=select_choices
                                shows_active_state_when_least_one_selected=true
                                shows_search_bar=true
                                search_placeholder="Nom d'une techno."
                                search_icon=IconSet::Search
                                reset_callback
                            />
                        }
                    })
                })}
            </ErrorBoundary>
        </Suspense>
    }
}
