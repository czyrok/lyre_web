use leptos::prelude::*;

use super::{button_theme::ButtonTheme, icon_side::IconSide};
use crate::{
    core::{component_size::ComponentSize, icon_set::IconSet},
    shared::components::icon::Icon,
};

#[component]
pub fn UnthemedButtonAsLink(
    theme: ButtonTheme,
    size: ComponentSize,
    text: String,
    href: String,
    #[prop(into)] icon: Option<IconSet>,
    #[prop(into)] icon_side: Option<IconSide>,
) -> impl IntoView {
    let is_accentuation_theme = theme == ButtonTheme::Accentuation;
    let is_primary_theme = theme == ButtonTheme::Primary;
    let is_secondary_theme = theme == ButtonTheme::Secondary;

    let is_xl_size = size == ComponentSize::XL;
    let is_lg_size = size == ComponentSize::LG;
    let is_md_size = size == ComponentSize::MD;
    let is_sm_size = size == ComponentSize::SM;

    let icon_side = icon_side.unwrap_or_default();

    let left_icon = icon.clone();
    let right_icon = icon.clone();

    let has_left_icon = icon.is_some() && icon_side == IconSide::Left;
    let has_right_icon = icon.is_some() && icon_side == IconSide::Right;

    view! {
        <a
            class=(["tw-accentuation-button"], move || is_accentuation_theme)
            class=(["tw-primary-button"], move || is_primary_theme)
            class=(["tw-secondary-button"], move || is_secondary_theme)

            class=(["tw-button-size-xl"], move || is_xl_size)
            class=(["tw-button-size-lg"], move || is_lg_size)
            class=(["tw-button-size-md"], move || is_md_size)
            class=(["tw-button-size-sm"], move || is_sm_size)

            href=href
        >
            {move || has_left_icon.then(|| {
                view! {
                    <span class="tw-button-icon">
                        <Icon icon=left_icon.clone().unwrap() />
                    </span>
                }
            })}

            <span class="tw-button-text">{ text }</span>

            {move || has_right_icon.then(|| {
                view! {
                    <span class="tw-button-icon">
                        <Icon icon=right_icon.clone().unwrap() />
                    </span>
                }
            })}
        </a>
    }
}
