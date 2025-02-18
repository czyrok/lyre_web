use leptos::{ev::MouseEvent, prelude::*};

use super::icon_side::IconSide;
use crate::{
    core::{component_size::ComponentSize, icon_set::IconSet},
    shared::components::button::{
        button_theme::ButtonTheme, unthemed_button::UnthemedButton,
    },
};

#[component]
pub fn SecondaryButton(
    size: ComponentSize,
    text: String,
    on_click: impl FnMut(MouseEvent) + 'static,
    #[prop(optional, into)] icon: Option<IconSet>,
    #[prop(optional, into)] icon_side: Option<IconSide>,
) -> impl IntoView {
    view! {
        <UnthemedButton theme=ButtonTheme::Secondary size=size text=text on_click=on_click icon=icon icon_side=icon_side />
    }
}
