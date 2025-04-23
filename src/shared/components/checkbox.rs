use leptos::prelude::*;

use crate::{
    core::data::icon_set::IconSet,
    shared::{components::icon::Icon, enums::component_size::ComponentSize},
};

#[component]
pub fn Checkbox(
    size: ComponentSize,
    text: String,
    #[prop(name = "value")] (is_checked, set_is_checked): (
        ReadSignal<bool>,
        WriteSignal<bool>,
    ),
    #[prop(default = true)] can_user_unchecked: bool,
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

                // FIXME:
                // on:input:target=move |event| {
                //     let is_checked = event.target().checked();

                //     if is_checked {
                //         set_is_checked.set(is_checked);

                //         return;
                //     }

                //     if can_user_unchecked {
                //         set_is_checked.set(is_checked);
                //     } else {
                //         //// We need to restore the checked view
                //         set_is_checked.set(true);
                //     }
                // }
                // prop:checked=is_checked
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
