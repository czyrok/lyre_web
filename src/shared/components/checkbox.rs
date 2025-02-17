use leptos::prelude::*;

use crate::{
    core::{component_size::ComponentSize, icon_set::IconSet},
    shared::components::icon::Icon,
};

#[component]
pub fn Checkbox(
    size: ComponentSize,
    text: String,
    #[prop(name = "value")] (is_toggled, set_is_toggled): (
        ReadSignal<bool>,
        WriteSignal<bool>,
    ),
) -> impl IntoView {
    let is_xl_size = size == ComponentSize::XL;
    let is_lg_size = size == ComponentSize::LG;
    let is_md_size = size == ComponentSize::MD;
    let is_sm_size = size == ComponentSize::SM;

    view! {
        <label
            class="tw-secondary-checkbox"
            class=(["tw-checkbox-size-xl"], move || is_xl_size)
            class=(["tw-checkbox-size-lg"], move || is_lg_size)
            class=(["tw-checkbox-size-md"], move || is_md_size)
            class=(["tw-checkbox-size-sm"], move || is_sm_size)
        >
            <input
                class="tw-checkbox-input"
                type="checkbox"

                on:input:target=move |event| {
                    set_is_toggled.set(event.target().checked());
                }
                prop:checked=is_toggled
            />

            <div class="tw-checkbox-box">
                <span class="tw-box-icon">
                    <Icon icon=IconSet::Check />
                </span>
            </div>

            <span class="tw-checkbox-text">{ text }</span>
        </label>
    }
}
