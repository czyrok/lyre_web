use leptos::prelude::*;

use crate::shared::{
    button::components::{
        accentuation_button_as_link::AccentuationButtonAsLink,
        secondary_button_as_link::SecondaryButtonAsLink,
    },
    components::theme_selector::ThemeSelector,
    enums::component_size::ComponentSize,
};

#[component]
pub fn Footer<TIntoView>(
    actions_render: impl Fn() -> TIntoView,
) -> impl IntoView
where
    TIntoView: IntoView,
{
    view! {
        <div class="tw-footer">
            <div class="tw-footer-more-part">
                <div class="tw-more-part-text">
                    <span>"Hop ! Hop !! Hop !!!"</span>
                    <br/><span>"Fin de la page."</span>
                </div>

                <div class="tw-more-part-actions">
                    <AccentuationButtonAsLink size=ComponentSize::MD text="Me Conctacter" href="/#contact" />

                    {actions_render()}

                    <SecondaryButtonAsLink size=ComponentSize::MD text="Revenir au début" href="#top" />
                </div>
            </div>

            <div class="tw-footer-bottom-part">
                <hr class="tw-bottom-part-separator" />

                <div class="tw-bottom-part-settings">
                    <ThemeSelector />
                </div>

                <div class="tw-bottom-part-copyright-text">
                    "Made with 🥐 in Lyon / © 2025 Dylan Valentin."
                </div>
            </div>
        </div>
    }
}
