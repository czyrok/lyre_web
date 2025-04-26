use leptos::prelude::*;
use leptos_router::components::A;

use crate::{core::data::icon_set::IconSet, shared::components::icon::Icon};

#[component]
pub fn NavBarItem(
    #[prop(into)] text: String,
    #[prop(into)] href: String,
    icon: IconSet,
    #[prop(default = true)] uses_active_behavior: bool,
) -> impl IntoView {
    view! {
        <A href exact=true>
            <span
                class="tw-nav-bar-item"
                class=("tw-nav-bar-item-active-behavior", uses_active_behavior)
            >
                <span class="tw-item-icon">
                    <Icon icon=icon />
                </span>

                <span class="tw-item-text">{ text }</span>
             </span>
        </A>
    }
}
