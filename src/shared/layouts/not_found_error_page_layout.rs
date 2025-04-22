use leptos::prelude::*;

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
    view! {
        <div class="tw-secondary-page-layout tw-not-found-error-page-layout">
            <div id="top" class="tw-anchor"></div>

            <div class="tw-secondary-page-layout-intro">
                <Brand size=ComponentSize::LG layout_mode=LayoutMode::BadgeOnly />
            </div>

            <div class="tw-secondary-page-layout-content">
                <h1 class="tw-title-size-lg">{title.clone()}</h1>

                <div class="tw-not-found-error-page-layout-home-back">
                    <p>"Tu t’es perdu ? Pas de problèmes."</p>

                    <AccentuationButtonAsLink size=ComponentSize::XL text="Retour à l'Accueil" href="/" />
                </div>
            </div>

            <Footer displays_actions=false />
        </div>
    }
}
