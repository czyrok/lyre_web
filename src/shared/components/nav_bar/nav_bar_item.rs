use leptos::prelude::*;
use leptos_router::components::A;

use crate::{core::icon_set::IconSet, shared::components::icon::Icon};

#[component]
pub fn NavBarItem(text: String, href: String, icon: IconSet) -> impl IntoView {
    view! {
        <A href=href exact=true>
            <span class="tw-nav-bar-item">
                <span class="tw-item-icon">
                    <Icon icon=icon />
                </span>

                <span class="tw-item-text">{ text }</span>
             </span>
        </A>
    }
}
