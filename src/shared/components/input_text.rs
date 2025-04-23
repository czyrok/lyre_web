use leptos::prelude::*;

use crate::{
    core::data::icon_set::IconSet,
    shared::{components::icon::Icon, enums::component_size::ComponentSize},
};

#[derive(Default, PartialEq)]
pub enum IconSide {
    #[default]
    Left,
    Right,
}

#[component]
pub fn InputText(
    size: ComponentSize,
    #[prop(name = "text")] (text, set_text): (
        ReadSignal<String>,
        WriteSignal<String>,
    ),
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional, into)] icon: Option<IconSet>,
    #[prop(optional, into)] icon_side: Option<IconSide>,
    #[prop(optional, into)] additional_style_classes: Option<String>,
) -> impl IntoView {
    let is_xl_size = size == ComponentSize::XL;
    let is_lg_size = size == ComponentSize::LG;
    let is_md_size = size == ComponentSize::MD;
    let is_sm_size = size == ComponentSize::SM;

    let icon_side = icon_side.unwrap_or_default();

    let left_icon = icon.clone();
    let right_icon = icon.clone();

    let has_left_icon = icon.is_some() && icon_side == IconSide::Left;
    let has_right_icon = icon.is_some() && icon_side == IconSide::Right;

    let style_classes = format!(
        "tw-accentuation-input-text tw-input-size-lg {}",
        additional_style_classes.unwrap_or_default()
    );

    view! {
        <label
            class=style_classes
            class=(["tw-input-size-xl"], move || is_xl_size)
            class=(["tw-input-size-lg"], move || is_lg_size)
            class=(["tw-input-size-md"], move || is_md_size)
            class=(["tw-input-size-sm"], move || is_sm_size)
        >
            {move || has_left_icon.then(|| {
                view! {
                    <span class="tw-input-text-icon">
                        <Icon icon=left_icon.clone().unwrap() />
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

            {move || has_right_icon.then(|| {
                view! {
                    <span class="tw-input-text-icon">
                        <Icon icon=right_icon.clone().unwrap() />
                    </span>
                }
            })}
        </label>
    }
}
