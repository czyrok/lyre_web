use leptos::prelude::*;

#[component]
pub fn ProjectCardSkeleton() -> impl IntoView {
    view! {
        <div class="tw-project-card-skeleton">
            <div class="tw-card-thumbnail-skeleton"></div>
            <div class="tw-card-title-skeleton"></div>
            <div class="tw-card-tag-skeleton-container">
                <div class="tw-card-tag-skeleton"></div>
                <div class="tw-card-tag-skeleton"></div>
                <div class="tw-card-tag-skeleton"></div>
            </div>
        </div>
    }
}
