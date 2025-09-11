use std::{fmt::Debug, hash::Hash};

use leptos::{prelude::*, web_sys::js_sys};

use crate::{
    core::data::icon_set::IconSet,
    shared::{
        components::dropdown_menu_search_bar::{
            DropdownMenuSearchBar, IconSide,
        },
        select::types::select_item::SelectItem,
    },
};

#[derive(PartialEq)]
pub enum Position {
    Right,
    Bottom,
}

#[derive(Clone)]
pub enum MenuState {
    Opened,
    Closed,
}

#[component]
pub fn DropdownMenu<IItemId>(
    position: Position,
    #[prop(into)] id: String,
    #[prop(into)] position_anchor_name: String,
    #[prop(default = false)] shows_search_bar: bool,
    #[prop(default = signal("".into()), name = "searched_text")]
    (searched_text, set_searched_text): (
        ReadSignal<String>,
        WriteSignal<String>,
    ),
    #[prop(default = None)] search_placeholder: Option<String>,
    #[prop(default = None)] search_icon: Option<IconSet>,
    #[prop(default = None)] search_icon_side: Option<IconSide>,
    items: ReadSignal<Vec<SelectItem<IItemId>>>,
) -> impl IntoView
where
    IItemId: Hash + Eq + Clone + Send + Sync + Debug + 'static,
{
    let is_position_right = position == Position::Right;
    let is_position_bottom = position == Position::Bottom;

    let is_initial_state =
        move || items.get().is_empty() && searched_text.get().is_empty();

    let is_empty_search_state =
        move || items.get().is_empty() && !searched_text.get().is_empty();

    let (current_state, set_current_state) = signal(MenuState::Closed);

    Effect::new(move || {
        if let MenuState::Closed = current_state.get() {
            set_searched_text.set("".into());
        }
    });

    view! {
        <div
            class="tw-dropdown-menu"
            class=(["tw-dropdown-menu-right"], move || is_position_right)
            class=(["tw-dropdown-menu-bottom"], move || is_position_bottom)

            id=id
            style=format!("position-anchor: --{}", position_anchor_name)
            role="menu"
            popover

            on:toggle=move |event| {
                // Source: https://developer.mozilla.org/en-US/docs/Web/API/ToggleEvent
                let is_open = js_sys::Reflect::get(&event, &"newState".into())
                    .unwrap()
                    .as_string()
                    .unwrap() == "open";

                if is_open {
                    set_current_state.set(MenuState::Opened);
                } else {
                    set_current_state.set(MenuState::Closed);
                }
            }
        >
            {move || shows_search_bar.then(|| {
                view! {
                    <DropdownMenuSearchBar searched_text set_searched_text menu_state=current_state placeholder=search_placeholder.clone() icon=search_icon.clone() icon_side=search_icon_side.clone() />
                }
            })}

            <div class="tw-dropdown-menu-items">
                {move || is_initial_state().then(|| {
                    view! {
                        <div class="tw-dropdown-menu-item">
                            <span class="tw-additional-info">
                                "Commencer par rechercher."
                            </span>
                        </div>
                    }
                })}

                {move || is_empty_search_state().then(|| {
                    view! {
                        <div class="tw-dropdown-menu-item">
                            <span class="tw-additional-info">
                                "Aucun résultat."
                            </span>
                        </div>
                    }
                })}

                <For
                    each=move || items.get()
                    key=|item| item.id.clone()
                    let:item
                >
                    <div class="tw-dropdown-menu-item">
                        {(item.view_creator_callback)()}
                    </div>
                </For>
            </div>
        </div>
    }
}
