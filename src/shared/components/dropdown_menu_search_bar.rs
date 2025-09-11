use leptos::{html, prelude::*};

use super::dropdown_menu::MenuState;
use crate::{core::data::icon_set::IconSet, shared::components::icon::Icon};

#[derive(Default, PartialEq, Clone)]
pub enum IconSide {
    #[default]
    Left,
    Right,
}

#[component]
pub fn DropdownMenuSearchBar(
    searched_text: ReadSignal<String>,
    set_searched_text: WriteSignal<String>,
    menu_state: ReadSignal<MenuState>,
    #[prop(default = None)] placeholder: Option<String>,
    #[prop(default = None)] icon: Option<IconSet>,
    #[prop(default = None)] icon_side: Option<IconSide>,
) -> impl IntoView {
    let icon_side = icon_side.unwrap_or_default();

    let left_icon = icon.clone();
    let right_icon = icon.clone();

    let has_left_icon = icon.is_some() && icon_side == IconSide::Left;
    let has_right_icon = icon.is_some() && icon_side == IconSide::Right;

    let input_ref: NodeRef<html::Input> = NodeRef::new();

    Effect::new(move || {
        if let MenuState::Opened = menu_state.get() {
            if let Some(input_element) = input_ref.get() {
                input_element.focus().unwrap();
            }
        }
    });

    view! {
        <label class="tw-dropdown-menu-search-bar">
            {move || has_left_icon.then(|| {
                view! {
                    <span class="tw-search-bar-text-icon">
                        <Icon icon=left_icon.clone().unwrap() />
                    </span>
                }
            })}

            <input
                class="tw-search-bar-text-input"
                type="text"
                placeholder=placeholder.unwrap_or_default()

                on:input:target=move |event| {
                    set_searched_text.set(event.target().value());
                }
                prop:value=searched_text

                node_ref=input_ref
            />

            {move || has_right_icon.then(|| {
                view! {
                    <span class="tw-search-bar-text-icon">
                        <Icon icon=right_icon.clone().unwrap() />
                    </span>
                }
            })}
        </label>
    }
}
