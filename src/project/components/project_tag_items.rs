use leptos::prelude::*;

use crate::project::{
    components::project_tag_item::ProjectTagItem,
    data::project_tags::ProjectTags,
};

#[component]
pub fn ProjectTagItems(
    project_tags: ProjectTags,
    uses_long_name: bool,
) -> impl IntoView {
    view! {
        <For each=move || project_tags.0.clone() key=|tag| tag.short_name.clone() let:tag>
            {move || {
                let mut name = tag.clone().short_name;

                if uses_long_name {
                    if let Some(long_name) = tag.clone().long_name {
                        name = long_name;
                    }
                }

                view! {
                    <ProjectTagItem text=name />
                }
            }}
        </For>
    }
}
