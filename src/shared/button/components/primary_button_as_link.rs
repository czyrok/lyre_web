use leptos::prelude::*;

use crate::{
    core::{component_size::ComponentSize, icon_set::IconSet},
    shared::button::{
        components::unthemed_button_as_link::UnthemedButtonAsLink,
        types::{button_theme::ButtonTheme, icon_side::IconSide},
    },
};

#[component]
pub fn PrimaryButtonAsLink(
    size: ComponentSize,
    #[prop(into)] text: String,
    #[prop(into)] href: String,
    #[prop(optional, into)] icon: Option<IconSet>,
    #[prop(optional, into)] icon_side: Option<IconSide>,
    #[prop(optional, into)] target: Option<String>,
) -> impl IntoView {
    view! {
        <UnthemedButtonAsLink theme=ButtonTheme::Primary size text href icon icon_side target />
    }
}
