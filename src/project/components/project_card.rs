use leptos::prelude::*;

use crate::project::{
    components::{
        project_tag::ProjectTag, project_thumbnail::ProjectThumbnail,
    },
    data::project_context::ProjectContext,
};

#[component]
pub fn ProjectCard(project_context: ProjectContext) -> impl IntoView {
    view! {
        <div class="tw-project-card">
            <ProjectThumbnail project_context=project_context.clone() />

            <span class="tw-card-title">
                { project_context.title }
            </span>

            <div class="tw-card-tag-container">
                <For each=move || project_context.tags.0.clone() key=|tag| tag.name.clone() let:tag>
                    <ProjectTag text=tag.name />
                </For>
            </div>
        </div>
    }
}
