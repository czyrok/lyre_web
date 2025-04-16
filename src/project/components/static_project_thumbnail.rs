use leptos::prelude::*;

use super::super::data::project_context::ProjectContext;
use crate::shared::{
    components::brand::{Brand, LayoutMode},
    enums::component_size::ComponentSize,
};

#[component]
pub fn StaticProjectThumbnail(
    project_context: ProjectContext,
) -> impl IntoView {
    view! {
        <div class="tw-project-thumbnail">
            <div class="tw-thumbnail-image">
                <img src=format!("/project_data_images/{}", project_context.image_url) alt="Screenshot of the project" />
            </div>

            <div class="tw-thumbnail-brand">
                <Brand size=ComponentSize::SM layout_mode=LayoutMode::BadgeOnly />
            </div>
        </div>
    }
}
