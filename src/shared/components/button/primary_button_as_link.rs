use leptos::prelude::*;

use super::icon_side::IconSide;
use crate::{
    core::{component_size::ComponentSize, icon_set::IconSet},
    shared::components::button::{
        button_theme::ButtonTheme,
        unthemed_button_as_link::UnthemedButtonAsLink,
    },
};

#[component]
pub fn PrimaryButtonAsLink(
    size: ComponentSize,
    text: String,
    href: String,
    icon: Option<IconSet>,
    icon_side: Option<IconSide>,
) -> impl IntoView {
    view! {
        <UnthemedButtonAsLink theme=ButtonTheme::Primary size=size text=text href=href icon=icon icon_side=icon_side />
    }
}
