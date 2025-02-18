use leptos::prelude::*;

use crate::{
    core::{component_size::ComponentSize, icon_set::IconSet},
    project::{
        api::project_context::get_relevant_project_contexts,
        components::{
            project_card::ProjectCard,
            project_card_skeleton::ProjectCardSkeleton,
        },
    },
    shared::components::button::{
        icon_side::IconSide, primary_button_as_link::PrimaryButtonAsLink,
    },
};

#[component]
pub fn HomeProjectSection() -> impl IntoView {
    let project_context_resource =
        OnceResource::new(get_relevant_project_contexts());

    let relevant_project_contexts = move || {
        project_context_resource
            .get()
            .map(|n| n.unwrap_or_default())
            .map(|project_contexts_dto| project_contexts_dto.project_contexts)
            .unwrap_or_default()
    };

    view! {
        <div class="tw-home-section-container tw-home-project-section">
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
                    <For each=relevant_project_contexts key=|project_context| project_context.slug.clone() let:project_context>
                        <ProjectCard project_context=project_context />
                    </For>
                </Suspense>
            </div>

            <div class="tw-section-actions">
                <PrimaryButtonAsLink size=ComponentSize::LG text="Explorer".into() href="/projects".into() icon=IconSet::Compass icon_side=IconSide::Left/>
            </div>
        </div>
    }
}
