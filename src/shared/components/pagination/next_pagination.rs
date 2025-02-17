use leptos::prelude::*;

use crate::{
    core::{component_size::ComponentSize, icon_set::IconSet},
    shared::components::button::{
        icon_side::IconSide, primary_button_as_link::PrimaryButtonAsLink,
    },
};

#[component]
pub fn NextPagination(
    click_text: String,
    href: String,
    secondary_text: Option<String>,
) -> impl IntoView {
    let secondary_text = secondary_text.unwrap_or("Le suivant.".into());

    view! {
        <div class="tw-pagination">
            <PrimaryButtonAsLink size=ComponentSize::MD text=click_text href=href icon=Some(IconSet::RightArrow) icon_side=Some(IconSide::Right) />

            <span class="tw-pagination-text">
                { secondary_text }
            </span>
        </div>
    }
}
