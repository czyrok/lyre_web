use std::fmt::Debug;

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
}
