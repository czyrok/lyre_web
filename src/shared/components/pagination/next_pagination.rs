use leptos::prelude::*;

use crate::{
    core::data::icon_set::IconSet,
    shared::{
        button::{
            components::primary_button_as_link::PrimaryButtonAsLink,
            types::icon_side::IconSide,
        },
        enums::component_size::ComponentSize,
    },
};

#[component]
pub fn NextPagination(
    click_text: String,
    href: String,
    #[prop(optional, into)] secondary_text: Option<String>,
) -> impl IntoView {
    let secondary_text = secondary_text.unwrap_or("Le suivant.".into());

    view! {
        <div class="tw-pagination">
            <PrimaryButtonAsLink size=ComponentSize::MD text=click_text href=href icon=IconSet::RightArrow icon_side=IconSide::Right />

            <span class="tw-pagination-text">
                { secondary_text }
            </span>
        </div>
    }
}
