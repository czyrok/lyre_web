use std::fmt::Debug;

use leptos::prelude::{GetUntracked, RwSignal, Set};

use super::{
    select_choice::SelectChoice, select_choices::SelectChoices,
    select_choices_behavior::SelectChoicesBehavior,
};

#[derive(Clone)]
pub struct MultiSelectChoices<TKey: Eq + Clone + Send + Sync + Debug + 'static>
{
    wrapped_select_choices: SelectChoices<TKey>,
    pub selected_choice_keys: RwSignal<Vec<TKey>>,
}

impl<TKey> MultiSelectChoices<TKey>
where
    TKey: Eq + Clone + Send + Sync + Debug + 'static,
{
    pub fn new(
        choices: Vec<SelectChoice<TKey>>,
        selected_choice_keys: RwSignal<Vec<TKey>>,
    ) -> Self {
        let select_choices = SelectChoices::new(choices);

        let multi_select = Self {
            wrapped_select_choices: select_choices,
            selected_choice_keys,
        };

        multi_select.update_selected_choice_keys_from_updates();

        multi_select
    }

    fn update_selected_choice_keys_from_updates(&self) {
        let self_clone = self.clone();

        for choice in self.wrapped_select_choices.choices.clone() {
            choice.attach(move |choice| {
                let mut current_selected_keys =
                    self_clone.selected_choice_keys.clone().get_untracked();

                if choice.is_checked.get_untracked() {
                    current_selected_keys.push(choice.key);
                } else {
                    let choice_key_index_in_current = current_selected_keys
                        .iter()
                        .position(|key| *key == choice.key);

                    if let Some(index) = choice_key_index_in_current {
                        current_selected_keys.swap_remove(index);
                    }
                }

                self_clone.selected_choice_keys.set(current_selected_keys);
            });
        }
    }
}

impl<TKey> SelectChoicesBehavior for MultiSelectChoices<TKey>
where
    TKey: Eq + Clone + Send + Sync + Debug + 'static,
{
    type Key = TKey;

    fn list(&self) -> Vec<SelectChoice<TKey>> {
        self.wrapped_select_choices.list()
    }

    fn change_all_status(
        &self,
        is_checked: bool,
        filter: Option<Box<dyn Fn(&SelectChoice<TKey>) -> bool>>,
    ) {
        self.wrapped_select_choices
            .change_all_status(is_checked, filter);
    }

    fn attach_consistency_behavior(&self) {}
}
