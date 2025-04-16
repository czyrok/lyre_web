use leptos::prelude::*;

use crate::shared::enums::component_size::ComponentSize;

#[derive(Default, PartialEq)]
pub enum LayoutMode {
    #[default]
    Full,
    BadgeOnly,
    NameOnly,
}

#[component]
pub fn Brand(
    size: ComponentSize,
    #[prop(optional, into)] layout_mode: Option<LayoutMode>,
) -> impl IntoView {
    let is_xl_size = size == ComponentSize::XL;
    let is_lg_size = size == ComponentSize::LG;
    let is_sm_size = size == ComponentSize::SM;

    let layout_mode = layout_mode.unwrap_or_default();

    let has_badge =
        layout_mode == LayoutMode::BadgeOnly || layout_mode == LayoutMode::Full;
    let has_name =
        layout_mode == LayoutMode::NameOnly || layout_mode == LayoutMode::Full;

    view! {
        <div
            class="tw-brand"

            class=(["tw-brand-size-xl"], move || is_xl_size)
            class=(["tw-brand-size-lg"], move || is_lg_size)
            class=(["tw-brand-size-sm"], move || is_sm_size)
        >
            {move || has_badge.then(|| {
                view! {
                    <div class="tw-brand-badge">
                        <div class="tw-badge-logo">
                            <svg width="100%" height="100%" viewBox="0 0 150 99" fill="none" xmlns="http://www.w3.org/2000/svg">
                                <path
                                    d="M0 0H23.2128C31.6872 0 39.0564 1.9344 45.3201 5.8032C51.676 9.57988 56.558 15.1989 59.9663 22.6601C63.4666 30.0292 65.2168 38.8722 65.2168 49.189C65.2168 59.5059 63.4666 68.3488 59.9663 75.7179C56.558 83.087 51.676 88.66 45.3201 92.4367C39.0564 96.2133 31.6872 98.1017 23.2128 98.1017H0V0ZM22.5219 86.7716C32.1939 86.7716 39.6551 83.5937 44.9056 77.2378C50.1561 70.7898 52.7814 61.4403 52.7814 49.189C52.7814 36.8458 50.1561 27.4501 44.9056 21.0021C39.6551 14.554 32.1939 11.3301 22.5219 11.3301H11.8827V86.7716H22.5219ZM73.7294 0H86.4411L113.523 90.3641H110.207L137.288 0H150L119.602 98.1017H104.127L73.7294 0Z"
                                    fill="currentColor" />
                            </svg>
                        </div>
                    </div>
                }
            })}

            {move || has_name.then(|| {
                view! {
                    <span class="tw-brand-name">
                        "D. Valentin"
                    </span>
                }
            })}
        </div>
    }
}
