use leptos::prelude::*;

use crate::{
    core::component_size::ComponentSize,
    shared::{
        button::components::secondary_button_as_link::SecondaryButtonAsLink,
        components::{
            brand::{Brand, LayoutMode},
            footer::Footer,
        },
    },
};

#[component]
pub fn SecondaryPageLayout<TContentRenderFunction, TContentIntoView>(
    #[prop(optional)] intro_render: Option<Box<dyn Fn() -> AnyView>>,
    content_render: TContentRenderFunction,
) -> impl IntoView
where
    TContentRenderFunction: Fn() -> TContentIntoView,
    TContentIntoView: IntoView,
{
    let intro_render = match intro_render {
        Some(intro_render) => intro_render,
        None => Box::new(move || {
            view! {
                <Brand size=ComponentSize::LG layout_mode=LayoutMode::BadgeOnly />
            }
            .into_any()
        }),
    };

    view! {
        <div class="tw-secondary-page-layout">
            <div id="top" class="tw-anchor"></div>

            <div class="tw-secondary-page-layout-intro">
                {intro_render()}
            </div>

            <div class="tw-secondary-page-layout-content">
                {content_render()}
            </div>

            <Footer actions_render=|| view! {
                <SecondaryButtonAsLink size=ComponentSize::MD text="Accueil" href="/" />
            } />
        </div>
    }
}
