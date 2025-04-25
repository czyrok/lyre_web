use leptos::{ev::MouseEvent, prelude::*};

use super::super::types::{
    button_action::ButtonAction, button_theme::ButtonTheme, icon_side::IconSide,
};
use crate::{
    core::data::icon_set::IconSet,
    shared::{components::icon::Icon, enums::component_size::ComponentSize},
};

#[component]
pub fn UnthemedButton(
    theme: ButtonTheme,
    size: ComponentSize,
    #[prop(into, default = "".into())] text: String,
    #[prop(into)] on_click: ButtonAction,
    #[prop(into)] icon: Option<IconSet>,
    #[prop(into)] icon_side: Option<IconSide>,
    #[prop(into, default = "".into())] anchor_name: String,
    #[prop(into)] shows_ping: Option<Signal<bool>>,
    #[prop(into)] is_errored: Option<Signal<bool>>,
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

    let mut on_click_callback: Option<Box<dyn FnMut(MouseEvent)>> = None;
    let mut popover_target_id: String = "".into();

    match on_click {
        ButtonAction::Callback(callback) => {
            on_click_callback = Some(callback);
        }
        ButtonAction::Popover(target_id) => {
            popover_target_id = target_id;
        }
        _ => (),
    };

    let has_text = !text.is_empty();

    let shows_ping = shows_ping.unwrap_or(signal(false).0.into());

    let is_errored = is_errored.unwrap_or(signal(false).0.into());

    if let Some(on_click_callback) = on_click_callback {
        return view! {
            <button
                class=(["tw-accentuation-button"], move || is_accentuation_theme)
                class=(["tw-primary-button"], move || is_primary_theme)
                class=(["tw-secondary-button"], move || is_secondary_theme)

                class=(["tw-button-size-xl"], move || is_xl_size)
                class=(["tw-button-size-lg"], move || is_lg_size)
                class=(["tw-button-size-md"], move || is_md_size)
                class=(["tw-button-size-sm"], move || is_sm_size)

                class=(["tw-button-ping"], move || shows_ping.get())

                class=(["tw-button-errored"], move || is_errored.get())
                disabled=is_errored.get()

                on:click=on_click_callback
                popovertarget=popover_target_id
                style=format!("anchor-name: --{}", anchor_name)
            >
                {move || has_left_icon.then(|| {
                    view! {
                        <span class="tw-button-icon">
                            <Icon icon=left_icon.clone().unwrap() />
                        </span>
                    }
                })}

                {move || has_text.then(|| {
                    view! {
                        <span class="tw-button-text">{ text.clone() }</span>
                    }
                })}

                {move || has_right_icon.then(|| {
                    view! {
                        <span class="tw-button-icon">
                            <Icon icon=right_icon.clone().unwrap() />
                        </span>
                    }
                })}
            </button>
        }.into_any();
    }

    view! {
        <button
            class=(["tw-accentuation-button"], move || is_accentuation_theme)
            class=(["tw-primary-button"], move || is_primary_theme)
            class=(["tw-secondary-button"], move || is_secondary_theme)

            class=(["tw-button-size-xl"], move || is_xl_size)
            class=(["tw-button-size-lg"], move || is_lg_size)
            class=(["tw-button-size-md"], move || is_md_size)
            class=(["tw-button-size-sm"], move || is_sm_size)

            class=(["tw-button-ping"], move || shows_ping.get())

            class=(["tw-button-errored"], move || is_errored.get())
            disabled=is_errored.get()

            popovertarget=popover_target_id
            style=format!("anchor-name: --{}", anchor_name)
        >
            {move || has_left_icon.then(|| {
                view! {
                    <span class="tw-button-icon">
                        <Icon icon=left_icon.clone().unwrap() />
                    </span>
                }
            })}

            {move || has_text.then(|| {
                view! {
                    <span class="tw-button-text">{ text.clone() }</span>
                }
            })}

            {move || has_right_icon.then(|| {
                view! {
                    <span class="tw-button-icon">
                        <Icon icon=right_icon.clone().unwrap() />
                    </span>
                }
            })}
        </button>
    }
    .into_any()
}
