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
}

impl<TKey> SelectChoices<TKey>
where
    TKey: Eq + Clone + Send + Sync + Debug + 'static,
{
    pub fn new(choices: Vec<SelectChoice<TKey>>) -> Self {
        Self { choices }
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
}
