use leptos::prelude::*;

use crate::shared::{
    components::brand::{Brand, LayoutMode},
    enums::component_size::ComponentSize,
};

#[component]
pub fn ProjectThumbnailSkeleton(
    #[prop(default = false)] displays_brand: bool,
) -> impl IntoView {
    view! {
        <div class="tw-project-thumbnail-skeleton">
            <div class="tw-thumbnail-skeleton-image" />

            {move || displays_brand.then(|| {
                view! {
                    <div class="tw-thumbnail-skeleton-brand">
                        <Brand size=ComponentSize::SM layout_mode=LayoutMode::BadgeOnly />
                    </div>
                }
            })}
        </div>
    }
}
