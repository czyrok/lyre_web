use leptos::prelude::*;

use super::super::types::theme::InputTextTheme;
use crate::{
    core::{data::icon_set::IconSet, types::closure::OnClickCallback},
    shared::{
        components::icon::Icon, enums::component_size::ComponentSize,
        input_text::types::state::InputTextState,
    },
};

#[component]
pub fn UnthemedInputText(
    theme: InputTextTheme,
    size: ComponentSize,
    #[prop(into)] placeholder: Option<String>,
    #[prop(into)] icon: Option<IconSet>,
    #[prop(into)] additional_style_classes: Option<String>,

    #[prop(name = "text")] (text, set_text): (
        ReadSignal<String>,
        WriteSignal<String>,
    ),
    #[prop(into)] reset_callback: Option<Box<dyn OnClickCallback>>,
    state: Signal<InputTextState>,
) -> impl IntoView {
    let is_primary_theme = theme == InputTextTheme::Primary;

    let is_xl_size = size == ComponentSize::XL;
    let is_lg_size = size == ComponentSize::LG;
    let is_md_size = size == ComponentSize::MD;
    let is_sm_size = size == ComponentSize::SM;

    let has_icon = icon.is_some();

    let reset_callback: Box<dyn OnClickCallback> =
        reset_callback.unwrap_or(Box::new(|_| {}));

    let is_active =
        Signal::derive(move || matches!(state.get(), InputTextState::Active));

    let style_classes = additional_style_classes.unwrap_or_default();

    view! {
        <label
            class=style_classes

            class=(["tw-primary-input-text"], move || is_primary_theme)

            class=(["tw-input-size-xl"], move || is_xl_size)
            class=(["tw-input-size-lg"], move || is_lg_size)
            class=(["tw-input-size-md"], move || is_md_size)
            class=(["tw-input-size-sm"], move || is_sm_size)
        >
            <span class="tw-input-text-left-group">
                {move || has_icon.then(|| {
                    view! {
                        <span class="tw-input-text-icon">
                            <Icon icon=icon.clone().unwrap() />
                        </span>
                    }
                })}

                <input
                    class="tw-input-text-input"
                    type="text"
                    placeholder=placeholder.unwrap_or_default()

                    on:input:target=move |event| {
                        set_text.set(event.target().value());
                    }
                    prop:value=text
                />
            </span>

            {move || is_active.get().then(|| {
                let mut reset_callback: Box<dyn OnClickCallback> =
                    dyn_clone::clone_box(&* reset_callback);

                view! {
                    <span class="tw-input-text-icon" on:click=move |event| {
                        event.prevent_default();
                        reset_callback(event)
                    } >
                        <Icon icon=IconSet::Cross />
                    </span>
                }
            })}
        </label>
    }
}
