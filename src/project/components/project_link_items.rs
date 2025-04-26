use leptos::prelude::*;

use crate::{
    project::data::project_links::ProjectLinks,
    shared::{
        button::{
            components::secondary_button_as_link::SecondaryButtonAsLink,
            types::icon_side::IconSide,
        },
        enums::component_size::ComponentSize,
    },
};

#[component]
pub fn ProjectLinkItems(project_links: ProjectLinks) -> impl IntoView {
    view! {
        <For each=move || project_links.0.clone() key=|link| link.url.clone() let:link>
            {move || {
                let link = link.clone();

                if link.icon.is_some() {
                    view! {
                        <SecondaryButtonAsLink size=ComponentSize::MD text=link.title href=link.url icon=link.icon.unwrap() icon_side=IconSide::Right target="_blank" />
                    }
                } else {
                    view! {
                        <SecondaryButtonAsLink size=ComponentSize::MD text=link.title href=link.url target="_blank" />
                    }
                }}
            }
        </For>
    }
}
