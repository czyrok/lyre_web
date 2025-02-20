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
        let select_choices = SelectChoices::new(choices);

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

    fn attach_consistency_behavior(&self) {
        let choices = self.0.choices.iter();

        for choice in choices {
            let mut current_choices = self.clone();

            choice.attach(move |updated_choice| {
                let new_value = updated_choice.is_checked.get();

                if new_value {
                    current_choices.0.change_all_status(false, |choice| {
                        choice.key != updated_choice.key.clone()
                    });
                }
            });
        }
    }
}
