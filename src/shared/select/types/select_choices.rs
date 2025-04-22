use std::fmt::Debug;

use leptos::prelude::*;

use super::select_choice::SelectChoice;

pub type SelectChoiceFilterCallback<TKey> =
    Box<dyn Fn(&SelectChoice<TKey>) -> bool>;

#[derive(Clone)]
pub struct SelectChoices<TKey>
where
    TKey: Eq + Clone + Send + Sync + Debug + 'static,
{
    pub choices: Vec<SelectChoice<TKey>>,
    pub selected_choice_keys: Option<RwSignal<Vec<TKey>>>,
}

impl<TKey> SelectChoices<TKey>
where
    TKey: Eq + Clone + Send + Sync + Debug + 'static,
{
    pub fn new(
        choices: Vec<SelectChoice<TKey>>,
        selected_choice_keys: Option<RwSignal<Vec<TKey>>>,
    ) -> Self {
        Self {
            choices,
            selected_choice_keys,
        }
    }

    pub fn change_all_status(
        &self,
        is_checked: bool,
        filter: Option<SelectChoiceFilterCallback<TKey>>,
    ) {
        let mut filtered_choices: Vec<SelectChoice<TKey>> = match filter {
            Some(filter) => {
                self.choices.clone().into_iter().filter(filter).collect()
            }
            None => self.choices.clone().into_iter().filter(|_| true).collect(),
        };

        for choice in filtered_choices.iter_mut() {
            choice.set_is_checked.maybe_update(|value| {
                if is_checked == *value {
                    return false;
                }

                *value = is_checked;

                true
            });
        }
    }

    pub fn list(&self) -> Vec<SelectChoice<TKey>> {
        self.choices.clone()
    }

    pub fn update_selected_choice_keys_from_updates(&self) {
        if let Some(selected_choice_keys) = self.selected_choice_keys {
            for choice in self.list() {
                choice.attach(move |choice| {
                    let mut current_selected_keys =
                        selected_choice_keys.clone().get_untracked();

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

                    selected_choice_keys.set(current_selected_keys);
                });
            }
        }
    }

    pub fn get_selected_choice_keys(&self) -> Option<Signal<Vec<TKey>>> {
        self.selected_choice_keys
            .map(|selected_choice_keys| selected_choice_keys.into())
    }
}
