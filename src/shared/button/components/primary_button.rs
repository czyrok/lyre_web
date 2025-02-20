use leptos::prelude::*;

use crate::{
    core::{component_size::ComponentSize, icon_set::IconSet},
    shared::button::{
        components::unthemed_button::UnthemedButton,
        types::{
            button_action::ButtonAction, button_theme::ButtonTheme,
            icon_side::IconSide,
        },
    },
};

#[component]
pub fn PrimaryButton(
    size: ComponentSize,
    text: String,
    #[prop(into)] on_click: ButtonAction,
    #[prop(optional, into)] icon: Option<IconSet>,
    #[prop(optional, into)] icon_side: Option<IconSide>,
    #[prop(into, default = "".into())] anchor_name: String,
) -> impl IntoView {
    view! {
        <UnthemedButton theme=ButtonTheme::Primary size text on_click icon icon_side anchor_name />
    }
}
