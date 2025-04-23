use leptos::prelude::*;
use leptos_router::components::A;

use crate::{
    core::data::icon_set::IconSet,
    project::data::project_context::ProjectContext,
    shared::{
        components::{
            brand::{Brand, LayoutMode},
            icon::Icon,
        },
        enums::component_size::ComponentSize,
    },
};

#[component]
pub fn ProjectThumbnail(
    project_context: ProjectContext,
    #[prop(default = false)] displays_brand: bool,
    #[prop(default = true)] enables_hover: bool,
) -> impl IntoView {
    let mut view = view! {
        <div class="tw-project-thumbnail">
            <div class="tw-thumbnail-image">
                <img src=format!("/project_data_images/{}", project_context.image_url) alt="Screenshot of the project" />
            </div>

            {move || displays_brand.then(|| {
                view! {
                    <div class="tw-thumbnail-brand">
                        <Brand size=ComponentSize::SM layout_mode=LayoutMode::BadgeOnly />
                    </div>
                }
            })}
        </div>
    }.into_any();

    if enables_hover {
        view = view! {
            <A href=format!("/projects/{}/", project_context.slug.expect("`slug` should exist"))>
                <div class="tw-project-thumbnail">
                    <div class="tw-thumbnail-image">
                        <img src=format!("/project_data_images/{}", project_context.image_url) alt="Screenshot of the project" />
                    </div>

                    <div class="tw-thumbnail-hover">
                        <span class="tw-thumbnail-hover-icon">
                            <Icon icon=IconSet::Eye />
                        </span>
                    </div>

                    {move || displays_brand.then(|| {
                        view! {
                            <div class="tw-thumbnail-brand">
                                <Brand size=ComponentSize::SM layout_mode=LayoutMode::BadgeOnly />
                            </div>
                        }
                    })}
                </div>
            </A>
        }.into_any()
    }

    view
}
