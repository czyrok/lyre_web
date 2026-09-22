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
        <div class="project-card">
            <ProjectThumbnail project_context=project_context.clone() />

            <span class="card-title">
                { project_context.title }
            </span>

            <div class="card-tag-container">
                <ProjectTagItems project_tags=project_context.tags uses_long_name=false />
            </div>
        </div>
    }
}
