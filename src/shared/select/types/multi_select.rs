use std::fmt::Debug;

use leptos::prelude::{RwSignal, Signal};

use super::{
    select_choice::SelectChoice, select_choices::SelectChoices,
    select_choices_behavior::SelectChoicesBehavior,
};

#[derive(Clone)]
pub struct MultiSelectChoices<TKey: Eq + Clone + Send + Sync + Debug + 'static>(
    SelectChoices<TKey>,
);

impl<TKey> MultiSelectChoices<TKey>
where
    TKey: Eq + Clone + Send + Sync + Debug + 'static,
{
    pub fn new(
        choices: Vec<SelectChoice<TKey>>,
        selected_choice_keys: RwSignal<Vec<TKey>>,
    ) -> Self {
        let select_choices =
            SelectChoices::new(choices, selected_choice_keys.into());

        let multi_select = Self(select_choices);

        multi_select.update_selected_choice_keys_from_updates();

        multi_select
    }
}

impl<TKey> SelectChoicesBehavior for MultiSelectChoices<TKey>
where
    TKey: Eq + Clone + Send + Sync + Debug + 'static,
{
    type Key = TKey;

    fn list(&self) -> Vec<SelectChoice<TKey>> {
        self.0.list()
    }

    fn change_all_status(
        &self,
        is_checked: bool,
        filter: Option<Box<dyn Fn(&SelectChoice<TKey>) -> bool>>,
    ) {
        self.0.change_all_status(is_checked, filter);
    }

    fn attach_consistency_behavior(&self) {}

    fn update_selected_choice_keys_from_updates(&self) {
        self.0.update_selected_choice_keys_from_updates();
    }

    fn get_selected_choice_keys(&self) -> Option<Signal<Vec<TKey>>> {
        self.0.get_selected_choice_keys()
    }
}
