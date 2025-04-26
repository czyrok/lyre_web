use std::fmt::Debug;

use leptos::prelude::*;

use super::{
    select_choice::SelectChoice, select_choices::SelectChoices,
    select_choices_behavior::SelectChoicesBehavior,
};

#[derive(Clone)]
pub struct SingleOptionalSelectChoices<
    TKey: Eq + Clone + Send + Sync + Debug + 'static,
>(SelectChoices<TKey>);

impl<TKey> SingleOptionalSelectChoices<TKey>
where
    TKey: Eq + Clone + Send + Sync + Debug + 'static,
{
    pub fn new(choices: Vec<SelectChoice<TKey>>) -> Self {
        let select_choices = SelectChoices::new(choices, None);

        Self(select_choices)
    }
}

impl<TKey> SelectChoicesBehavior for SingleOptionalSelectChoices<TKey>
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
        let choices = self.0.choices.iter();

        for choice in choices {
            let current_choices = self.clone();

            choice.attach(move |updated_choice: SelectChoice<TKey>| {
                let new_value = updated_choice.is_checked.get();

                if new_value {
                    current_choices.0.change_all_status(
                        false,
                        Some(Box::new(move |choice: &SelectChoice<TKey>| {
                            choice.key != updated_choice.key
                        })),
                    );
                }
            });
        }
    }

    fn update_selected_choice_keys_from_updates(&self) {
        self.0.update_selected_choice_keys_from_updates();
    }

    fn get_selected_choice_keys(&self) -> Option<Signal<Vec<TKey>>> {
        self.0.get_selected_choice_keys()
    }
}
