use leptos::{prelude::*, Params};
use leptos_router::{hooks::use_params, params::Params};

use crate::{
    project::{
        components::{
            project_details_actions::ProjectDetailsActions,
            project_details_error_display::ProjectDetailsErrorDisplay,
            project_details_footer::ProjectDetailsFooter,
            project_details_page_skeleton::ProjectDetailsSkeleton,
            project_details_paginator::ProjectDetailsPaginator,
            project_tag_items::ProjectTagItems,
            project_thumbnail::ProjectThumbnail,
        },
        resources::project_details_resource::ProjectDetailsResource,
    },
    shared::{
        components::link::Link, enums::component_size::ComponentSize,
        layouts::secondary_page_layout::SecondaryPageLayout,
    },
};

#[derive(Params, Clone, Debug, PartialEq, Eq)]
struct ProjectDetailsParams {
    slug: Option<String>,
}

#[component]
pub fn ProjectDetails() -> impl IntoView {
    let route_params = use_params::<ProjectDetailsParams>();

    let project_slug = Memo::new(move |_| {
        route_params
            .get()
            .map(|q| q.slug.unwrap_or_default())
            .unwrap_or_default()
    });

    let resource = ProjectDetailsResource::new(project_slug.into());

    view! {
        <Suspense fallback=move || view! {
            <ProjectDetailsSkeleton />
        }>
            <ErrorBoundary fallback=|errors| {
                view! {
                    <ProjectDetailsErrorDisplay errors />
                }
            }>
                {move || Suspend::new(async move {
                    resource.get_project().await.map(|project| {
                        let project_context = project.context.clone();
                        let next_project = project.context.next.clone();

                        view! {
                            <SecondaryPageLayout
                                intro_renderer=Box::new(move || view! {
                                    <ProjectThumbnail project_context=project_context.clone() enables_hover=false displays_brand=true />
                                }.into_any())

                                content_renderer=move || view! {
                                    <div class="tw-project-details-page-top-part">
                                        <div class="tw-top-part-intro">
                                            <Link size=ComponentSize::MD text="Mes Projets/" href="/projects" />

                                            <div class="tw-intro-details">
                                                <h1 class="tw-title-size-lg">{ project.context.title.clone() }</h1>

                                                <div class="tw-details-tags">
                                                    <ProjectTagItems project_tags=project.context.tags.clone() />
                                                </div>
                                            </div>
                                        </div>

                                        <ProjectDetailsActions project_links=project.links.clone() />
                                    </div>

                                    <div class="tw-project-details-page-middle-part">
                                        <div class="tw-middle-part-text">
                                            <div class="tw-text-content" inner_html=project.content.0.clone() />

                                            <span class="tw-additional-info">{project.context.formatted_date.clone().expect("`formatted_date` should exist")}</span>
                                        </div>
                                    </div>

                                    <ProjectDetailsPaginator next_project=project.context.next.clone() />
                                }.into_any()

                                footer_renderer=Box::new(move || {
                                    view! {
                                        <ProjectDetailsFooter next_project=next_project.clone() />
                                    }.into_any()
                                })
                            />
                        }
                    })
                })}
            </ErrorBoundary>
        </Suspense>
    }
}
