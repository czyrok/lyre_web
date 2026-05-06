use std::{fmt::Debug, hash::Hash};

use leptos::prelude::*;

use crate::shared::select::types::{
    select_choices_behavior::SelectChoicesBehavior, select_state::SelectState,
};

pub fn effect_active_state<TChoiceKey>(
    select_choices: &impl SelectChoicesBehavior<Key = TChoiceKey>,
    select_state: RwSignal<SelectState>,
) where
    TChoiceKey: Hash + Eq + Clone + Send + Sync + Debug + 'static,
{
    let has_selected_choices = select_choices
        .get_selected_choice_keys()
        .map(|selected_choice_keys| {
            Signal::derive(move || {
                let selected_choice_keys = selected_choice_keys.get();

                !selected_choice_keys.is_empty()
            })
        })
        .unwrap_or(signal(false).0.into());

    Effect::new(move || {
        match select_state.get() {
            SelectState::Default => {
                let has_selected_choices = has_selected_choices.get();

                if has_selected_choices {
                    select_state.set(SelectState::Active);
                }
            }
            SelectState::Active => {
                let has_selected_choices = has_selected_choices.get();

                if !has_selected_choices {
                    select_state.set(SelectState::Default);
                }
            }
            _ => (),
        };
    });
}
