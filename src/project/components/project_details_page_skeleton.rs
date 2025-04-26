use leptos::prelude::*;

use crate::{
    project::components::project_thumbnail_skeleton::ProjectThumbnailSkeleton,
    shared::{
        components::{footer::Footer, link::Link},
        enums::component_size::ComponentSize,
        layouts::secondary_page_layout::SecondaryPageLayout,
    },
};

#[component]
pub fn ProjectDetailsSkeleton() -> impl IntoView {
    view! {
        <SecondaryPageLayout
            intro_renderer=Box::new(move || view! {
                <ProjectThumbnailSkeleton displays_brand=true />
            }.into_any())

            content_renderer=move || view! {
                <div class="tw-project-details-page-top-part">
                    <div class="tw-top-part-intro">
                        <Link size=ComponentSize::MD text="Mes Projets/" href="/projects" />

                        <div class="tw-intro-details">
                            <div class="tw-h1-skeleton tw-title-size-lg"></div>

                            <div class="tw-details-tags">
                                <div class="tw-project-tag-skeleton"></div>
                                <div class="tw-project-tag-skeleton"></div>
                                <div class="tw-project-tag-skeleton"></div>
                            </div>
                        </div>
                    </div>

                    <div class="tw-top-part-actions">
                        <div class="tw-secondary-button-skeleton tw-button-size-md"></div>
                        <div class="tw-secondary-button-skeleton tw-button-size-md"></div>
                    </div>
                </div>

                <div class="tw-project-details-page-middle-part">
                    <div class="tw-middle-part-text">
                        <div class="tw-text-content">
                            <div class="tw-p-skeleton"></div>
                            <div class="tw-p-skeleton"></div>
                            <div class="tw-p-skeleton"></div>
                            <div class="tw-p-skeleton"></div>
                            <div class="tw-p-skeleton"></div>
                            <div class="tw-p-skeleton"></div>
                        </div>
                    </div>
                </div>
            }.into_any()

            footer_renderer=Box::new(move || view! {
                <Footer displays_actions=false />
            }.into_any())
        />
    }
}
