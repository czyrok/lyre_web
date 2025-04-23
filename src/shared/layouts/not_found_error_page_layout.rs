#[cfg(feature = "ssr")]
use http::StatusCode;
use leptos::prelude::*;
#[cfg(feature = "ssr")]
use leptos_axum::ResponseOptions;

use crate::shared::{
    button::components::accentuation_button_as_link::AccentuationButtonAsLink,
    components::{
        brand::{Brand, LayoutMode},
        footer::Footer,
    },
    enums::component_size::ComponentSize,
};

#[component]
pub fn NotFoundErrorPageLayout(
    #[prop(default = "Page non trouvée".into(), into)] title: String,
) -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        let response = expect_context::<ResponseOptions>();

        response.set_status(StatusCode::NOT_FOUND);
    }

    view! {
        <div class="tw-secondary-page-layout tw-not-found-error-page-layout">
            <div id="top" class="tw-anchor"></div>

            <div class="tw-secondary-page-layout-intro">
                <Brand size=ComponentSize::LG layout_mode=LayoutMode::BadgeOnly />
            </div>

            <main class="tw-secondary-page-layout-content">
                <h1 class="tw-title-size-lg">{title.clone()}</h1>

                <div class="tw-not-found-error-page-layout-home-back">
                    <p>"Tu t’es perdu ? Pas de problème."</p>

                    <AccentuationButtonAsLink size=ComponentSize::XL text="Retour à l'Accueil" href="/" />
                </div>
            </main>

            <Footer displays_actions=false />
        </div>
    }
}
