use leptos::prelude::*;
use leptos_meta::*;
use leptos_use::signal_debounced;

use crate::{
    core::{
        data::fetch_state::FetchState,
        dto::cursor_pagination_dto::CursorPaginationDto,
    },
    project::{
        components::{
            ordered_project_context_filter::OrderedProjectContextFilter,
            ordered_project_context_paginator::OrderedProjectContextPaginator,
            project_card::ProjectCard,
            project_card_skeleton::ProjectCardSkeleton,
            search_result_info::SearchResultInfo,
        },
        data::project_context::ProjectContext,
        dto::project_context_filter_dto::ProjectContextFilterDto,
        resources::ordered_project_contexts_resource::OrderedProjectContextsResource,
    },
    shared::{
        button::components::secondary_button_as_link::SecondaryButtonAsLink,
        components::footer::Footer, enums::component_size::ComponentSize,
        layouts::secondary_page_layout::SecondaryPageLayout,
    },
};

#[component]
pub fn ProjectSearchPage() -> impl IntoView {
    let (project_contexts, set_project_contexts): (
        ReadSignal<Vec<ProjectContext>>,
        WriteSignal<Vec<ProjectContext>>,
    ) = signal(vec![]);

    let (pagination, set_pagination) = signal(CursorPaginationDto::default());
    let (project_context_filter, set_project_context_filter) =
        signal(ProjectContextFilterDto::default());

    let (is_loading, set_is_loading) = signal(true);
    let delayed_is_loading: Signal<bool> = signal_debounced(is_loading, 500.0);

    let displayed_project_contexts =
        Memo::new(move |previous_value: Option<&Vec<ProjectContext>>| {
            let is_loading = is_loading.get();
            let is_loading_deb = delayed_is_loading.get();

            if is_loading != is_loading_deb {
                if let Some(previous_value) = previous_value {
                    return previous_value.clone();
                }
            }

            project_contexts.get()
        });

    let first_project_context = Signal::derive(move || {
        let project_contexts = displayed_project_contexts.get();

        project_contexts.first().cloned()
    });

    let (last_fetch_state, set_last_fetch_state) =
        signal(FetchState::default());
    let resource = OrderedProjectContextsResource::new(
        pagination.into(),
        project_context_filter.into(),
        set_is_loading,
    );

    Effect::new(move || {
        let displayed_project_contexts = project_contexts.get_untracked();

        let aggregated_project_contexts = resource
            .aggregate_fetched_project_contexts(displayed_project_contexts);

        set_project_contexts.set(aggregated_project_contexts.clone());

        //// First case, when this static page is loaded directly, WASM is not loaded,
        //// so, `is_loading` equals to `true` until it's loaded. In this case,
        //// the resource is ready since the beginning because it's a static page
        //// and the content is theoretically already fetched (contained in the page).
        //// This `effect` will run once the WASM is loaded.
        ////
        //// Second case: when the user navigates to this page from another,
        //// the WASM is loaded and this `effect` runs two times. First time, when the user
        //// reaches the page and a second time when the resource gets a response from
        //// the server. So as to prevent `is_loading` equals to `true` the first running time,
        //// we must check if the resource is ready.
        if resource.is_ready_untracked() {
            set_is_loading.set(false);
        }
    });

    Effect::new(move || {
        let fetch_state = resource.get_fetch_state();

        set_last_fetch_state.set(fetch_state);
    });

    let reset_view_when_filter_updated = move || {
        set_project_contexts.set(vec![]);
        set_pagination.set(CursorPaginationDto::default());
    };

    let displays_info = Signal::derive(move || {
        let delayed_is_loading = delayed_is_loading.get();
        let last_fetch_error = last_fetch_state.get();
        let project_contexts = displayed_project_contexts.get();

        !delayed_is_loading
            && (!last_fetch_error.is_ok() || project_contexts.is_empty())
    });

    let displays_list_block = Signal::derive(move || {
        let delayed_is_loading = delayed_is_loading.get();
        let last_fetch_error = last_fetch_state.get();
        let project_contexts = displayed_project_contexts.get();

        !delayed_is_loading
            && last_fetch_error.is_ok()
            && !project_contexts.is_empty()
    });

    view! {
        <Title text="Mes Projets | Dylan Valentin" />

        <Meta name="author" content="Dylan Valentin" />
        <Meta name="description" content="Développeur Fullstack en freelance, je transforme les idées en solutions techniques. Mon parcours, de Kepler à Ubikap, allie passion pour la tech et flexibilité du freelance. Découvrez mes projets !" />
        <Meta name="keywords" content="
        design, conception, développement web, web assembly, Rust, ElectronJS, navigateur web, fonctionnalités, système de mise à jour,
        jeu en ligne multijoueur, Les loups-garous de Thiercelieux, architecture, backend, workers NodeJS, instances Cloud, JWT, authentification, méthode agile, travail en équipe, communication,
        logiciel SaaS, Ubikap, développement frontend, développement backend, DevOps, administration système, infrastructure, sécurité, automatisation, déploiements, maintenance, polyvalence,
        stack technique, Leptos, framework web assembly, challenge design, harmonie des couleurs, veille technique, optimisation, sécurité des applications,
        gestion de fichiers, sauvegarde, RxJS, parallélisme,
        formation, réécriture de projet, qualité de code, maintenance, contact client, retours utilisateurs,
        framework, abstraction, encapsulation, Express.js, Socket.io, décorateurs TypeScript, injection de dépendance, typage, NestJS, choix techniques,
        microcontrôleur, développement microcontrôleur, capteurs,
        IOT, SBC, Session Border Controller, téléphonie SIP, séquences programmées
        " />

        <Meta property="og:title" content="Mes Projets | Dylan Valentin" />
        <Meta property="og:description" content="Développeur Fullstack en freelance, je transforme les idées en solutions techniques. Mon parcours, de Kepler à Ubikap, allie passion pour la tech et flexibilité du freelance. Découvrez mes projets !" />

        <SecondaryPageLayout
            content_renderer=move || view! {
                <div class="tw-project-search-page-top-part">
                    <h1 class="tw-title-size-lg">"Mes Projets"</h1>
                </div>

                <div class="tw-project-search-page-middle-part">
                    <OrderedProjectContextFilter default_filter=project_context_filter.get_untracked() on_update=move |project_context_filter| {
                        set_project_context_filter.set(project_context_filter);
                        reset_view_when_filter_updated();
                    } />

                    <Show when=move || { displays_info.get() }>
                        <SearchResultInfo last_fetch_state=last_fetch_state.into() project_contexts=displayed_project_contexts.into() />
                    </Show>

                    <Show when=move || { displays_list_block.get() }>
                        <div class="tw-middle-part-list">
                            // TODO: recycler view...
                            <For each=move || displayed_project_contexts.get() key=|project_context| project_context.slug.clone() let:project_context>
                                <ProjectCard project_context=project_context />
                            </For>
                        </div>
                    </Show>

                    <Show when=move || { delayed_is_loading.get() }>
                        <div class="tw-middle-part-list">
                            <ProjectCardSkeleton />
                        </div>
                    </Show>
                </div>

                <Show when=move || { displays_list_block.get() }>
                    <OrderedProjectContextPaginator resource=resource project_contexts=displayed_project_contexts.into() set_pagination />
                </Show>
            }.into_any()

            footer_renderer=Box::new(move || view! {
                <Footer middle_action_renderer=
                    Box::new(move || view! {
                        <Show
                            when=move || { first_project_context.get().is_some() }
                            fallback=|| view! {
                                <SecondaryButtonAsLink size=ComponentSize::MD text="Accueil" href="/" />
                            }
                        >
                            <SecondaryButtonAsLink
                                size=ComponentSize::MD
                                text="Project au Hasard"
                                href=format!("/projects/{}/", first_project_context.get().unwrap().slug.expect("`slug` should exist"))
                            />
                        </Show>
                    }.into_any())
                />
            }.into_any())
        />
    }
}
