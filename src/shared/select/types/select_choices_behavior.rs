use std::fmt::Debug;

use super::select_choice::SelectChoice;

pub trait SelectChoicesBehavior {
    type Key: Eq + Clone + Send + Sync + Debug + 'static;

    fn list(&self) -> Vec<SelectChoice<Self::Key>>;
    fn attach_consistency_behavior(&self);
}
