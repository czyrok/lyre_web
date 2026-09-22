use std::{fmt::Debug, hash::Hash};

use leptos::prelude::*;

use crate::shared::select::types::select_choices_behavior::SelectChoicesBehavior;

pub fn selected_count_signal<TChoiceKey>(
    select_choices: &impl SelectChoicesBehavior<Key = TChoiceKey>,
) -> Signal<usize>
where
    TChoiceKey: Hash + Eq + Clone + Send + Sync + Debug + 'static,
{
    select_choices
        .get_selected_choice_keys()
        .map(|selected_choice_keys| {
            Signal::derive(move || selected_choice_keys.get().len())
        })
        .unwrap_or(signal(0).0.into())
}
