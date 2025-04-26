use leptos::prelude::*;

use crate::project::{
    components::{
        project_tag_items::ProjectTagItems, project_thumbnail::ProjectThumbnail,
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
                <ProjectTagItems project_tags=project_context.tags />
            </div>
        </div>
    }
}
