use leptos::prelude::*;

use crate::{
    core::data::fetch_state::FetchState,
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
    let (last_fetch_state, set_last_fetch_state) =
        signal(FetchState::default());
    let resource = RelevantProjectContextsResource::default();

    Effect::new(move || {
        let fetch_state = resource.get_fetch_state();

        set_last_fetch_state.set(fetch_state);
    });

    view! {
        <div class="tw-landing-page-section-container tw-landing-page-project-section">
            <div id="projects" class="tw-anchor"></div>

            <div class="tw-section-text">
                <h1 class="tw-title-size-xl">"Mes projets"</h1>

                <p>"Depuis mon début dans la programmation, j'ai réalisé plusieurs projets que ce soit dans le cadre de mes études ou personnel. Ces années de pratique tournent autour de plusieurs périodes de ma vie : le lycée, l'IUT et aujourd'hui le freelance. Cette sélection retrace ces différentes périodes de ma jeune carrière."</p>
            </div>

            <div class="tw-section-projects">
                // TODO: utiliser error boundary comme dans project_details
                <Suspense fallback=move || {
                    view! {
                        <ProjectCardSkeleton />
                        <ProjectCardSkeleton />
                        <ProjectCardSkeleton />
                    }
                }>
                    <FetchErrorDisplay fetch_state=last_fetch_state.into() />

                    <For each=move || resource.get_fetched_project_contexts() key=|project_context| project_context.slug.clone().expect("`slug` should exist") let:project_context>
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
