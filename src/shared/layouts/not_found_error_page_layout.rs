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
        <div class="secondary-page-layout not-found-error-page-layout">
            <div id="top" class="anchor"></div>

            <div class="secondary-page-layout-intro">
                <Brand size=ComponentSize::LG layout_mode=LayoutMode::BadgeOnly />
            </div>

            <main class="secondary-page-layout-content">
                <h1 class="title-size-lg">{title.clone()}</h1>

                <div class="not-found-error-page-layout-home-back">
                    <p>"Tu t’es perdu ? Pas de problème."</p>

                    <AccentuationButtonAsLink size=ComponentSize::XL text="Retour à l'Accueil" href="/" />
                </div>
            </main>

            <Footer displays_actions=false />
        </div>
    }
}
