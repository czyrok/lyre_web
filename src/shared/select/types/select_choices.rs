use std::fmt::Debug;

use leptos::prelude::*;

use super::select_choice::SelectChoice;

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

    pub fn change_all_status<TFilter>(
        &mut self,
        is_checked: bool,
        filter: TFilter,
    ) where
        TFilter: FnMut(&&mut SelectChoice<TKey>) -> bool,
    {
        let filtered_choices = self.choices.iter_mut().filter(filter);

        for choice in filtered_choices {
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
