use std::fmt::Debug;

use leptos::prelude::Signal;

use super::{
    select_choice::SelectChoice,
    select_choices_behavior::SelectChoicesBehavior,
    single_optional_select::SingleOptionalSelectChoices,
};

#[derive(Clone)]
pub struct SingleSelectChoices<TKey: Eq + Clone + Send + Sync + Debug + 'static>(
    SingleOptionalSelectChoices<TKey>,
);

impl<TKey> SingleSelectChoices<TKey>
where
    TKey: Eq + Clone + Send + Sync + Debug + 'static,
{
    pub fn new(mut choices: Vec<SelectChoice<TKey>>) -> Self {
        for choice in choices.iter_mut() {
            choice.can_user_unchecked = false;
        }

        let select_choices = SingleOptionalSelectChoices::new(choices);

        Self(select_choices)
    }
}

impl<TKey> SelectChoicesBehavior for SingleSelectChoices<TKey>
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

    fn attach_consistency_behavior(&self) {
        self.0.attach_consistency_behavior();
    }

    fn update_selected_choice_keys_from_updates(&self) {
        self.0.update_selected_choice_keys_from_updates();
    }

    fn get_selected_choice_keys(&self) -> Option<Signal<Vec<TKey>>> {
        self.0.get_selected_choice_keys()
    }
}
