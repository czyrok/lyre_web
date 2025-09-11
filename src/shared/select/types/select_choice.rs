use std::{fmt::Debug, hash::Hash, sync::Arc};

use leptos::prelude::*;

use super::select_item::SelectItem;
use crate::shared::{
    components::checkbox::Checkbox, enums::component_size::ComponentSize,
};

#[derive(Clone)]
pub struct SelectChoice<TKey>
where
    TKey: Hash + Eq + Clone + Send + Sync + Debug,
{
    pub key: TKey,
    pub text: String,
    pub is_checked: ReadSignal<bool>,
    pub set_is_checked: WriteSignal<bool>,
    pub can_user_unchecked: bool,
}

impl<TKey> SelectChoice<TKey>
where
    TKey: Hash + Eq + Clone + Send + Sync + Debug + 'static,
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

impl<TKey> From<SelectChoice<TKey>> for SelectItem<TKey>
where
    TKey: Hash + Eq + Clone + Send + Sync + Debug + 'static,
{
    fn from(choice: SelectChoice<TKey>) -> SelectItem<TKey> {
        SelectItem::new(
            choice.key.clone(),
            Arc::new(move || {
                let choice = choice.clone();

                view! {
                    <Checkbox size=ComponentSize::SM text=choice.text value=(choice.is_checked, choice.set_is_checked) can_user_unchecked=choice.can_user_unchecked />
                }.into_any()
            }),
        )
    }
}
