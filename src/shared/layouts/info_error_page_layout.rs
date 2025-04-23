use leptos::prelude::*;

use crate::shared::{
    components::{
        brand::{Brand, LayoutMode},
        footer::Footer,
    },
    enums::component_size::ComponentSize,
};

#[component]
pub fn InfoErrorPageLayout(
    content_renderer: impl Fn() -> AnyView,
) -> impl IntoView {
    view! {
        <div class="tw-secondary-page-layout tw-info-error-page-layout">
            <div id="top" class="tw-anchor"></div>

            <div class="tw-secondary-page-layout-intro">
                <Brand size=ComponentSize::LG layout_mode=LayoutMode::BadgeOnly />
            </div>

            <main class="tw-secondary-page-layout-content">
                {content_renderer()}
            </main>

            <Footer displays_actions=false />
        </div>
    }
}
