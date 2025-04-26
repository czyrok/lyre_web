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
pub fn FooterMorePart(
    middle_action_renderer: Option<Box<dyn Fn() -> AnyView>>,
) -> impl IntoView {
    let middle_action_renderer =
        middle_action_renderer.unwrap_or(Box::new(|| {
            view! {
                <SecondaryButtonAsLink size=ComponentSize::MD text="Accueil" href="/" />
            }
            .into_any()
        }));

    view! {
        <div class="tw-footer-more-part">
            <div class="tw-more-part-text">
                <span>"Hop ! Hop !! Hop !!!"</span>
                <br/><span>"Fin de la page."</span>
            </div>

            <div class="tw-more-part-actions">
                <AccentuationButtonAsLink size=ComponentSize::MD text="Me Conctacter" href="/#contact" />

                {middle_action_renderer()}

                <SecondaryButtonAsLink size=ComponentSize::MD text="Revenir au début" href="#top" />
            </div>
        </div>
    }
}

#[component]
pub fn Footer(
    #[prop(default = true)] displays_actions: bool,
    #[prop(optional)] middle_action_renderer: Option<Box<dyn Fn() -> AnyView>>,
) -> impl IntoView {
    let mut more_part_view = view! { "" }.into_any();

    if displays_actions {
        more_part_view = view! {
            <FooterMorePart middle_action_renderer />
        }
        .into_any();
    }

    view! {
        <div class="tw-footer">
            {more_part_view}

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
