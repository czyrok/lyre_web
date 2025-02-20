use std::fmt::Debug;

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

    fn attach_consistency_behavior(&self) {
        self.0.attach_consistency_behavior();
    }
}
