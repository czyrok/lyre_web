use leptos::prelude::*;

use super::super::types::{
    select_actions::SelectActions, select_state::SelectState,
};
use crate::{
    core::{data::icon_set::IconSet, types::closure::OnClickCallback},
    shared::{
        components::icon::Icon, enums::component_size::ComponentSize,
        select::types::select_theme::SelectTheme,
    },
};

#[component]
pub fn UnthemedSelectButton(
    theme: SelectTheme,
    size: ComponentSize,
    #[prop(into)] text: String,
    #[prop(into)] icon: Option<IconSet>,
    #[prop(into)] actions: SelectActions,
    #[prop(into)] anchor_name: String,
    state: Signal<SelectState>,
    #[prop(optional, into)] selected_count: Option<Signal<usize>>,
) -> impl IntoView {
    let is_primary_theme = theme == SelectTheme::Primary;
    let is_secondary_theme = theme == SelectTheme::Secondary;

    let is_xl_size = size == ComponentSize::XL;
    let is_lg_size = size == ComponentSize::LG;
    let is_md_size = size == ComponentSize::MD;
    let is_sm_size = size == ComponentSize::SM;

    let has_icon = icon.is_some();

    let popover_target_id: String = actions.popover_target_id.clone();
    let reset_callback: Box<dyn OnClickCallback> =
        actions.reset_callback.unwrap_or(Box::new(|_| {}));
    let on_click_callback: Box<dyn OnClickCallback> =
        actions.on_click_callback.unwrap_or(Box::new(|_| {}));

    let is_default =
        Signal::derive(move || matches!(state.get(), SelectState::Default));
    let is_active =
        Signal::derive(move || matches!(state.get(), SelectState::Active));
    let is_errored =
        Signal::derive(move || matches!(state.get(), SelectState::Errored(_)));

    let text = Signal::derive(move || match state.get() {
        SelectState::Errored(error_message) => error_message,
        _ => text.clone(),
    });

    view! {
        <button
            class=(["primary-select"], move || is_primary_theme)
            class=(["secondary-select"], move || is_secondary_theme)

            class=(["select-size-xl"], move || is_xl_size)
            class=(["select-size-lg"], move || is_lg_size)
            class=(["select-size-md"], move || is_md_size)
            class=(["select-size-sm"], move || is_sm_size)

            class=(["select-active"], move || is_active.get())
            class=(["select-errored"], move || is_errored.get())
            disabled=is_errored.get()

            on:click=on_click_callback
            popovertarget=popover_target_id
            style=format!("anchor-name: --{}", anchor_name)
        >
            <span class="select-left-group">
                {move || has_icon.then(|| {
                    view! {
                        <span class="select-icon">
                            <Icon icon=icon.clone().unwrap() />
                        </span>
                    }
                })}

                <span class="select-text-group">
                    <span class="select-text">{ text.get() }</span>

                    {move || selected_count.and_then(|selected_count| {
                        let count = selected_count.get();

                        (count > 0).then(|| {
                            view! {
                                <span class="select-count">{ count }</span>
                            }
                        })
                    })}
                </span>
            </span>

            {move || is_default.get().then(|| {
                view! {
                    <span class="select-icon">
                        <Icon icon=IconSet::SingleDownArrow />
                    </span>
                }
            })}

            {move || is_active.get().then(|| {
                let mut reset_callback: Box<dyn OnClickCallback> =
                    dyn_clone::clone_box(&* reset_callback);

                view! {
                    <span class="select-icon" on:click=move |event| {
                        event.prevent_default();
                        reset_callback(event)
                    } >
                        <Icon icon=IconSet::Cross />
                    </span>
                }
            })}
        </button>
    }
    .into_any()
}
