use leptos::prelude::*;

use crate::{
    project::data::next_project::NextProject,
    shared::{
        button::components::secondary_button_as_link::SecondaryButtonAsLink,
        components::footer::Footer, enums::component_size::ComponentSize,
    },
};

#[component]
pub fn ProjectDetailsFooter(
    next_project: Option<NextProject>,
) -> impl IntoView {
    let footer_action_renderer: Box<dyn Fn() -> AnyView> = next_project.map_or(
        Box::new(move || {
            view! {
                <SecondaryButtonAsLink size=ComponentSize::MD text="Accueil" href="/" />
            }
            .into_any()
        }),
        |next| {
        Box::new(move || {
            view! {
                <SecondaryButtonAsLink
                    size=ComponentSize::MD
                    text="Project Suivant"
                    href=format!("/projects/{}/", next.clone().slug)
                />
            }
            .into_any()
        })
    });

    view! {
        <Footer middle_action_renderer=footer_action_renderer />
    }
}
