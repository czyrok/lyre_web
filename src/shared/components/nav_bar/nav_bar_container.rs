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
                <NavBarItem text="Accueil".into() href="/".into() icon=IconSet::Eye />
                <NavBarItem text="Test".into() href="/projects".into() icon=IconSet::RightArrow />
            </nav>
        </div>
    }
}
