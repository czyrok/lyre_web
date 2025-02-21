use leptos::prelude::*;

use crate::{
    core::icon_set::IconSet,
    shared::components::nav_bar::nav_bar_item::NavBarItem,
};

#[component]
pub fn NavBarContainer() -> impl IntoView {
    view! {
        <div class="tw-nav-bar-wrapper">
            <nav class="tw-nav-bar">
                <NavBarItem text="Accueil".into() href="/".into() icon=IconSet::Home />
                <NavBarItem text="Mes Projets".into() href="/projects".into() icon=IconSet::Compass />
                <NavBarItem text="À Propos".into() href="/#about".into() icon=IconSet::About uses_active_behavior=false />
                <NavBarItem text="Me Contacter".into() href="/#contact".into() icon=IconSet::Email uses_active_behavior=false />
            </nav>
        </div>
    }
}
