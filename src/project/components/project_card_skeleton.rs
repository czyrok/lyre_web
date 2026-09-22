use leptos::prelude::*;

use crate::project::components::project_thumbnail_skeleton::ProjectThumbnailSkeleton;

#[component]
pub fn ProjectCardSkeleton() -> impl IntoView {
    view! {
        <div class="project-card-skeleton">
            <ProjectThumbnailSkeleton />

            <div class="card-skeleton-title" />

            <div class="card-skeleton-tag-container">
                <div class="project-tag-skeleton" />
                <div class="project-tag-skeleton" />
                <div class="project-tag-skeleton" />
            </div>
        </div>
    }
}
