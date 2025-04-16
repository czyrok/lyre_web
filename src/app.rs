use std::path::Path;

use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{FlatRoutes, Redirect, Route, Router},
    path,
    static_routes::StaticRoute,
    SsrMode,
};

use crate::{
    core::data::app_settings::UsesDarkTheme,
    landing_page::view::LandingPage,
    project::{
        api::project_slug_api::get_project_slugs,
        views::{
            project_details::ProjectDetails,
            project_search_page::ProjectSearchPage,
        },
    },
    shared::components::nav_bar::nav_bar_container::NavBarContainer,
    system::{
        state::frontend_contexts::use_app_settings, watch_path::watch_path,
    },
};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let (app_settings, _) = use_app_settings();

    let (uses_dark_theme, set_uses_dark_theme) =
        signal(UsesDarkTheme::default().0);

    //// This is useful to force rerender due to an hydration bug with static files
    Effect::new(move || {
        let current = app_settings.get().uses_dark_theme;

        set_uses_dark_theme.set(current.0);
    });

    let fallback = || {
        (view! {
          "Page not found."
        })
        .into_view()
    };

    view! {
        <Title text="Welcome to Leptos"/>

        <div
            id="style-settings"
            class=(["tw-dark"], uses_dark_theme)
        >
            <Router>
                <NavBarContainer />

                <main>
                    <FlatRoutes fallback>
                        <Route
                            path=path!("/")
                            view=LandingPage
                            ssr=SsrMode::Static(
                                StaticRoute::new().regenerate(|_| watch_path(Path::new("./project_data"))),
                            )
                        />

                        // TODO: y'a un intru
                        <Route
                            path=path!("/about")
                            view=move || view! { <Redirect path="/"/> }
                            ssr=SsrMode::Static(StaticRoute::new())
                        />

                        <Route
                            path=path!("/projects")
                            view=ProjectSearchPage
                            ssr=SsrMode::Static(StaticRoute::new())
                        />

                        <Route
                            path=path!("/projects/:slug/")
                            view=ProjectDetails
                            ssr=SsrMode::Static(
                                StaticRoute::new()
                                    .prerender_params(|| async move {
                                        [("slug".into(), get_project_slugs().await.unwrap_or_default())]
                                            .into_iter()
                                            .map(|params| (params.0, params.1.project_slugs))
                                            .collect()
                                    })
                                    .regenerate(|params| {
                                        let slug = params.get("slug").unwrap();
                                        watch_path(Path::new(&format!("./project_data/{slug}.md")))
                                    }),
                            )
                        />

                    </FlatRoutes>
                </main>
            </Router>
        </div>
    }
}
