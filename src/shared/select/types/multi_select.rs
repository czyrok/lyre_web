use std::fmt::Debug;

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
    pub fn new(choices: Vec<SelectChoice<TKey>>) -> Self {
        let select_choices = SelectChoices::new(choices);

        Self(select_choices)
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

    fn attach_consistency_behavior(&self) {}
}
