use std::fmt::Debug;

use leptos::prelude::*;

#[derive(Clone)]
pub struct SelectChoice<TKey>
where
    TKey: Eq + Clone + Send + Debug + Sync,
{
    pub key: TKey,
    pub text: String,
    pub is_checked: ReadSignal<bool>,
    pub set_is_checked: WriteSignal<bool>,
    pub can_user_unchecked: bool,
}

impl<TKey> SelectChoice<TKey>
where
    TKey: Eq + Clone + Send + Sync + Debug + 'static,
{
    pub fn new(key: TKey, text: String, is_checked: Option<bool>) -> Self {
        let (is_checked, set_is_checked) = signal(is_checked.unwrap_or(false));

        Self {
            key,
            text,
            is_checked,
            set_is_checked,
            can_user_unchecked: true,
        }
    }

    pub fn attach(
        &self,
        mut callback: impl FnMut(SelectChoice<TKey>) + 'static,
    ) {
        let current_choice = self.clone();

        Effect::new(move |_| {
            current_choice.is_checked.track();

            callback(current_choice.clone());
        });
    }
}
