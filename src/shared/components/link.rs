use leptos::prelude::*;
use leptos_router::components::A;

use crate::{
    core::{component_size::ComponentSize, icon_set::IconSet},
    shared::components::icon::Icon,
};

#[component]
pub fn Link(
    size: ComponentSize,
    text: String,
    href: String,
    is_external: bool,
) -> impl IntoView {
    let is_xl_size = size == ComponentSize::XL;
    let is_lg_size = size == ComponentSize::LG;
    let is_md_size = size == ComponentSize::MD;
    let is_sm_size = size == ComponentSize::SM;

    view! {
        <A href>
            <span
                class="tw-secondary-link"
                class=(["tw-link-size-xl"], move || is_xl_size)
                class=(["tw-link-size-lg"], move || is_lg_size)
                class=(["tw-link-size-md"], move || is_md_size)
                class=(["tw-link-size-sm"], move || is_sm_size)
            >
                <span class="tw-link-text">{ text }</span>

                {move || is_external.then(|| {
                    view! {
                        <span class="tw-link-icon">
                            <Icon icon=IconSet::External />
                        </span>
                    }
                })}
            </span>
        </A>
    }
}
