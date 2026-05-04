use std::{fmt::Debug, hash::Hash};

use leptos::prelude::*;

use crate::{
    core::data::icon_set::IconSet,
    shared::{
        components::{
            dropdown_menu::{DropdownMenu, Position},
            dropdown_menu_search_bar::IconSide as SearchBarIconSide,
        },
        select::{
            helpers::anchor::SelectAnchorNames,
            types::{
                select_choices_behavior::SelectChoicesBehavior,
                select_item::SelectItem,
            },
        },
    },
};

type FilteredChoicesSignals<TChoiceKey> = (
    ReadSignal<Vec<SelectItem<TChoiceKey>>>,
    WriteSignal<Vec<SelectItem<TChoiceKey>>>,
);

#[component]
pub fn SelectDropdownMenu<TChoiceKey>(
    anchor_names: SelectAnchorNames,
    dropdown_menu_position: Position,
    select_choices: impl SelectChoicesBehavior<Key = TChoiceKey>,
    shows_search_bar: bool,
    search_placeholder: Option<String>,
    search_icon: Option<IconSet>,
    search_icon_side: Option<SearchBarIconSide>,
) -> impl IntoView
where
    TChoiceKey: Hash + Eq + Clone + Send + Sync + Debug + 'static,
{
    let choices = select_choices.list();

    select_choices.attach_consistency_behavior();

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
            set_filtered_choices.set(
                choices
                    .clone()
                    .into_iter()
                    .map(|choice| choice.into())
                    .collect::<Vec<_>>(),
            );

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
        <DropdownMenu
            position=dropdown_menu_position
            id=anchor_names.dropdown_menu
            position_anchor_name=anchor_names.button
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
