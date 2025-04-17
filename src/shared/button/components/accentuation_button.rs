use leptos::prelude::*;

use crate::shared::{
    button::{
        components::unthemed_button::UnthemedButton,
        types::{
            button_action::ButtonAction, button_theme::ButtonTheme,
            icon_side::IconSide,
        },
    },
    components::icon::IconSet,
    enums::component_size::ComponentSize,
};

#[component]
pub fn AccentuationButton(
    size: ComponentSize,
    #[prop(into, default = "".into())] text: String,
    #[prop(into)] on_click: ButtonAction,
    #[prop(optional, into)] icon: Option<IconSet>,
    #[prop(optional, into)] icon_side: Option<IconSide>,
    #[prop(into, default = "".into())] anchor_name: String,
    #[prop(optional, into)] shows_ping: Option<Signal<bool>>,
) -> impl IntoView {
    view! {
        <UnthemedButton theme=ButtonTheme::Accentuation size text on_click icon icon_side anchor_name shows_ping />
    }
}
