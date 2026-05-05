use std::{fmt::Debug, hash::Hash};

use leptos::prelude::*;

use super::super::types::{
    select_actions::SelectActions, select_state::SelectState,
};
use crate::{
    core::{data::icon_set::IconSet, types::closure::OnClickCallback},
    shared::{
        components::{
            dropdown_menu::Position,
            dropdown_menu_search_bar::IconSide as SearchBarIconSide,
        },
        enums::component_size::ComponentSize,
        select::{
            components::{
                select_dropdown_menu::SelectDropdownMenu,
                unthemed_select_button::UnthemedSelectButton,
            },
            helpers::{
                active_state::effect_active_state, anchor::get_anchor_names,
            },
            types::{
                select_choices_behavior::SelectChoicesBehavior,
                select_theme::SelectTheme,
            },
        },
    },
};

#[component]
pub fn SecondarySelect<TChoiceKey>(
    size: ComponentSize,
    // TODO: faire une macro pour générer un uuid à la compile
    #[prop(into)] identifier: String,
    #[prop(into)] text: String,
    #[prop(optional, into)] icon: Option<IconSet>,

    select_choices: impl SelectChoicesBehavior<Key = TChoiceKey>,

    #[prop(into, optional)] state: Option<RwSignal<SelectState>>,
    #[prop(default = false)] shows_active_state_when_least_one_selected: bool,
    #[prop(into, optional)] on_click_callback: Option<Box<dyn OnClickCallback>>,
    #[prop(into, optional)] reset_callback: Option<Box<dyn OnClickCallback>>,

    dropdown_menu_position: Position,
    #[prop(default = false)] shows_search_bar: bool,
    #[prop(optional, into)] search_placeholder: Option<String>,
    #[prop(optional, into)] search_icon: Option<IconSet>,
    #[prop(optional, into)] search_icon_side: Option<SearchBarIconSide>,
) -> impl IntoView
where
    TChoiceKey: Hash + Eq + Clone + Send + Sync + Debug + 'static,
{
    let anchor_names = get_anchor_names(identifier);

    let state = state.unwrap_or(RwSignal::new(SelectState::Default));

    if shows_active_state_when_least_one_selected {
        effect_active_state(&select_choices, state)
    }

    let actions = SelectActions::new(
        anchor_names.clone().dropdown_menu,
        on_click_callback,
        reset_callback,
    );

    view! {
        <UnthemedSelectButton theme=SelectTheme::Secondary size text icon actions anchor_name=anchor_names.clone().button state=state.into() />

        <SelectDropdownMenu
            anchor_names
            dropdown_menu_position
            select_choices
            shows_search_bar
            search_placeholder
            search_icon
            search_icon_side
        >
        </SelectDropdownMenu>
    }
}
