use leptos::prelude::*;

use crate::project::components::project_thumbnail_skeleton::ProjectThumbnailSkeleton;

#[component]
pub fn ProjectCardSkeleton() -> impl IntoView {
    view! {
        <div class="tw-project-card-skeleton">
            <ProjectThumbnailSkeleton />

            <div class="tw-card-skeleton-title" />

            <div class="tw-card-skeleton-tag-container">
                <div class="tw-project-tag-skeleton" />
                <div class="tw-project-tag-skeleton" />
                <div class="tw-project-tag-skeleton" />
            </div>
        </div>
    }
}
