use leptos::prelude::*;

use crate::shared::{
    components::{
        brand::{Brand, LayoutMode},
        footer::Footer,
    },
    enums::component_size::ComponentSize,
};

#[component]
pub fn SecondaryPageLayout(
    #[prop(optional)] intro_renderer: Option<Box<dyn Fn() -> AnyView>>,
    content_renderer: impl Fn() -> AnyView,
    #[prop(optional)] footer_renderer: Option<Box<dyn Fn() -> AnyView>>,
) -> impl IntoView {
    let intro_renderer = intro_renderer.unwrap_or(Box::new(move || {
        view! {
            <Brand size=ComponentSize::LG layout_mode=LayoutMode::BadgeOnly />
        }
        .into_any()
    }));

    let footer_renderer = footer_renderer.unwrap_or(Box::new(move || {
        view! {
            <Footer />
        }
        .into_any()
    }));

    view! {
        <div class="tw-secondary-page-layout">
            <div id="top" class="tw-anchor"></div>

            <div class="tw-secondary-page-layout-intro">
                {intro_renderer()}
            </div>

            <main class="tw-secondary-page-layout-content">
                {content_renderer()}
            </main>

            {footer_renderer()}
        </div>
    }
}
