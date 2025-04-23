use leptos::prelude::*;

use crate::{
    core::data::icon_set::IconSet,
    shared::components::nav_bar::nav_bar_item::NavBarItem,
};

#[component]
pub fn NavBarContainer() -> impl IntoView {
    view! {
        <div class="tw-nav-bar-wrapper">
            <nav class="tw-nav-bar">
                <NavBarItem text="Accueil" href="/" icon=IconSet::Home />
                <NavBarItem text="Mes Projets" href="/projects" icon=IconSet::Compass />
                <NavBarItem text="À Propos" href="/#about" icon=IconSet::About uses_active_behavior=false />
                <NavBarItem text="Me Contacter" href="/#contact" icon=IconSet::Email uses_active_behavior=false />
            </nav>
        </div>
    }
}
