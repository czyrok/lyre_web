use std::{fmt::Debug, hash::Hash, sync::Arc};

use leptos::prelude::*;

#[derive(Clone)]
pub struct SelectItem<TId>
where
    TId: Hash + Eq + Clone + Send + Sync + Debug,
{
    pub id: TId,
    pub view_creator_callback: Arc<dyn Fn() -> AnyView + Send + Sync>,
}

impl<TId> SelectItem<TId>
where
    TId: Hash + Eq + Clone + Send + Sync + Debug + 'static,
{
    pub fn new(
        id: TId,
        view_creator_callback: Arc<dyn Fn() -> AnyView + Send + Sync>,
    ) -> Self {
        Self {
            id,
            view_creator_callback,
        }
    }
}
