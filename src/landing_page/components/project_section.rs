use leptos::prelude::*;

use crate::{
    core::error::server_function_error::ServerFunctionException,
    landing_page::resources::relevant_project_contexts_resource::RelevantProjectContextsResource,
    project::components::{
        project_card::ProjectCard, project_card_skeleton::ProjectCardSkeleton,
    },
    shared::{
        button::{
            components::primary_button_as_link::PrimaryButtonAsLink,
            types::icon_side::IconSide,
        },
        components::{fetch_error_display::FetchErrorDisplay, icon::IconSet},
        enums::component_size::ComponentSize,
    },
};

#[component]
pub fn ProjectSection() -> impl IntoView {
    let (last_fetch_error, set_last_fetch_error): (
        ReadSignal<Result<(), ServerFunctionException>>,
        WriteSignal<Result<(), ServerFunctionException>>,
    ) = signal(Ok(()));
    let resource = RelevantProjectContextsResource::new();

    Effect::new(move || {
        resource.track();

        if let Err(error) = resource.is_errored() {
            set_last_fetch_error.set(Err(error));
        }
    });

    view! {
        <div class="tw-landing-page-section-container tw-landing-page-project-section">
            <div id="projects" class="tw-anchor"></div>

            <div class="tw-section-text">
                <h1 class="tw-title-size-xl">"Mes projets"</h1>

                <p>"Depuis mon début dans la programmation, j'ai réalisé plusieurs projets que ce soit dans le cadre de mes études ou personnel. Ces années de pratique tournent autour de plusieurs périodes de ma vie : le lycée, l'IUT et aujourd'hui le freelance. Cette sélection retrace ces différentes périodes de ma jeune carrière."</p>
            </div>

            <div class="tw-section-projects">
                <Suspense fallback=move || {
                    view! {
                        <ProjectCardSkeleton />
                        <ProjectCardSkeleton />
                        <ProjectCardSkeleton />
                    }
                }>
                    <FetchErrorDisplay fetch_error=last_fetch_error.into() />

                    <For each=move || resource.get_fetched_project_contexts() key=|project_context| project_context.slug.clone() let:project_context>
                        <ProjectCard project_context=project_context />
                    </For>
                </Suspense>
            </div>

            <div class="tw-section-actions">
                <PrimaryButtonAsLink size=ComponentSize::LG text="Explorer" href="/projects" icon=IconSet::Compass icon_side=IconSide::Left/>
            </div>
        </div>
    }
}
