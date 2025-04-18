use leptos::prelude::*;

use crate::shared::{
    button::components::secondary_button_as_link::SecondaryButtonAsLink,
    components::{
        brand::{Brand, LayoutMode},
        footer::Footer,
    },
    enums::component_size::ComponentSize,
};

#[component]
pub fn SecondaryPageLayout(
    #[prop(optional)] intro_render: Option<Box<dyn Fn() -> AnyView>>,
    content_render: impl Fn() -> AnyView,
    #[prop(optional)] footer_actions_render: Option<Box<dyn Fn() -> AnyView>>,
) -> impl IntoView {
    let intro_render = intro_render.unwrap_or(Box::new(move || {
        view! {
            <Brand size=ComponentSize::LG layout_mode=LayoutMode::BadgeOnly />
        }
        .into_any()
    }));

    let footer_actions_render =
        footer_actions_render.unwrap_or(Box::new(move || {
            view! {
                <SecondaryButtonAsLink size=ComponentSize::MD text="Accueil" href="/" />
            }
            .into_any()
        }
    ));

    view! {
        <div class="tw-secondary-page-layout">
            <div id="top" class="tw-anchor"></div>

            <div class="tw-secondary-page-layout-intro">
                {intro_render()}
            </div>

            <div class="tw-secondary-page-layout-content">
                {content_render()}
            </div>

            <Footer actions_render=footer_actions_render />
        </div>
    }
}
