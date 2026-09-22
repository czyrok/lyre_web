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
                <div class="project-details-page-top-part">
                    <div class="top-part-intro">
                        <Link size=ComponentSize::MD text="Mes Projets/" href="/projects" />

                        <div class="intro-details">
                            <div class="h1-skeleton title-size-lg"></div>

                            <div class="details-tags">
                                <div class="project-tag-skeleton"></div>
                                <div class="project-tag-skeleton"></div>
                                <div class="project-tag-skeleton"></div>
                            </div>
                        </div>
                    </div>

                    <div class="top-part-actions">
                        <div class="secondary-button-skeleton button-size-md"></div>
                        <div class="secondary-button-skeleton button-size-md"></div>
                    </div>
                </div>

                <div class="project-details-page-middle-part">
                    <div class="middle-part-text">
                        <div class="text-content">
                            <div class="p-skeleton"></div>
                            <div class="p-skeleton"></div>
                            <div class="p-skeleton"></div>
                            <div class="p-skeleton"></div>
                            <div class="p-skeleton"></div>
                            <div class="p-skeleton"></div>
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
