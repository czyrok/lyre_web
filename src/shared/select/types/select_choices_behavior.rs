use std::fmt::Debug;

use leptos::prelude::Signal;

use super::{
    select_choice::SelectChoice, select_choices::SelectChoiceFilterCallback,
};

pub trait SelectChoicesBehavior {
    type Key: Eq + Clone + Send + Sync + Debug + 'static;

    fn list(&self) -> Vec<SelectChoice<Self::Key>>;
    fn change_all_status(
        &self,
        is_checked: bool,
        filter: Option<SelectChoiceFilterCallback<Self::Key>>,
    );
    fn attach_consistency_behavior(&self);
    fn update_selected_choice_keys_from_updates(&self);
    fn get_selected_choice_keys(&self) -> Option<Signal<Vec<Self::Key>>>;
}
