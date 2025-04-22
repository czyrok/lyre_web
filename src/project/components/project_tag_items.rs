use leptos::prelude::*;

use crate::project::{
    components::project_tag_item::ProjectTagItem,
    data::project_tags::ProjectTags,
};

#[component]
pub fn ProjectTagItems(project_tags: ProjectTags) -> impl IntoView {
    view! {
        <For each=move || project_tags.0.clone() key=|tag| tag.name.clone() let:tag>
            <ProjectTagItem text=tag.name />
        </For>
    }
}
