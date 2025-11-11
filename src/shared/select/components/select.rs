use std::{fmt::Debug, hash::Hash};

use leptos::prelude::*;

use crate::{
    core::data::icon_set::IconSet,
    shared::{
        button::{
            components::secondary_button::SecondaryButton,
            types::icon_side::IconSide,
        },
        components::{
            dropdown_menu::{DropdownMenu, Position},
            dropdown_menu_search_bar::IconSide as SearchBarIconSide,
        },
        enums::component_size::ComponentSize,
        select::types::{
            select_choices_behavior::SelectChoicesBehavior,
            select_item::SelectItem,
        },
    },
};

type FilteredChoicesSignals<TChoiceKey> = (
    ReadSignal<Vec<SelectItem<TChoiceKey>>>,
    WriteSignal<Vec<SelectItem<TChoiceKey>>>,
);

#[component]
pub fn Select<TChoiceKey>(
    size: ComponentSize,
    #[prop(optional, into)] icon: Option<IconSet>,
    #[prop(optional, into)] icon_side: Option<IconSide>,
    dropdown_menu_position: Position,
    #[prop(into)] text: String,
    // TODO: faire une macro pour générer un uuid à la compile
    #[prop(into)] identifier: String,
    select_choices: impl SelectChoicesBehavior<Key = TChoiceKey>,
    #[prop(default = false)] shows_ping_when_least_one_selected: bool,
    #[prop(default = false)] shows_search_bar: bool,
    #[prop(optional, into)] search_placeholder: Option<String>,
    #[prop(optional, into)] search_icon: Option<IconSet>,
    #[prop(optional, into)] search_icon_side: Option<SearchBarIconSide>,
) -> impl IntoView
where
    TChoiceKey: Hash + Eq + Clone + Send + Sync + Debug + 'static,
{
    let choices = select_choices.list();

    select_choices.attach_consistency_behavior();

    let drop_menu_anchor_name = format!("drop-menu-{identifier}");
    let button_anchor_name = format!("button-{identifier}");

    let icon = icon.unwrap_or(IconSet::SingleDownArrow);
    let icon_side = icon_side.unwrap_or(IconSide::Right);

    let shows_ping = select_choices
        .get_selected_choice_keys()
        .map(|selected_choice_keys| {
            if !shows_ping_when_least_one_selected {
                return signal(false).0.into();
            }

            Signal::derive(move || {
                let selected_choice_keys = selected_choice_keys.get();

                !selected_choice_keys.is_empty()
            })
        })
        .unwrap_or(signal(false).0.into());

    let searched_text: (ReadSignal<String>, WriteSignal<String>) =
        signal("".into());

    let (filtered_choices, set_filtered_choices): FilteredChoicesSignals<
        TChoiceKey,
    > = signal(vec![]);

    Effect::new(move |previous_value: Option<String>| {
        let searched_text = searched_text.0.get();
        let searched_text_parts = searched_text.split(" ");

        if let Some(previous_value) = previous_value {
            if searched_text == previous_value {
                return searched_text;
            }
        }

        if !shows_search_bar {
            set_filtered_choices.set(
                choices
                    .clone()
                    .into_iter()
                    .map(|choice| choice.into())
                    .collect::<Vec<_>>(),
            );

            return searched_text;
        }

        if searched_text.is_empty() {
            set_filtered_choices.set(vec![]);

            return searched_text;
        }

        let mut new_filtered_choices = choices
            .clone()
            .into_iter()
            .filter(move |choice| {
                for part in searched_text_parts.clone() {
                    if part.is_empty() {
                        continue;
                    }

                    if choice.text.to_lowercase().contains(&part.to_lowercase())
                    {
                        return true;
                    }
                }

                false
            })
            .map(|choice| choice.into())
            .collect::<Vec<_>>();

        new_filtered_choices.truncate(4);

        set_filtered_choices.set(new_filtered_choices);

        searched_text
    });

    view! {
        <SecondaryButton size text on_click=drop_menu_anchor_name.clone() icon icon_side anchor_name=button_anchor_name.clone() shows_ping />

        <DropdownMenu
            position=dropdown_menu_position
            id=drop_menu_anchor_name
            position_anchor_name=button_anchor_name
            shows_search_bar
            searched_text
            search_placeholder
            search_icon
            search_icon_side
            items=filtered_choices
        >
        </DropdownMenu>
    }
}
