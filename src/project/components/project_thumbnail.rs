use leptos::prelude::*;
use leptos_router::components::A;

use crate::{
    project::data::project_context::ProjectContext,
    shared::components::icon::{Icon, IconSet},
};

#[component]
pub fn ProjectThumbnail(project_context: ProjectContext) -> impl IntoView {
    view! {
        <A href=format!("/projects/{}/", project_context.slug)>
            <div class="tw-project-thumbnail">
                <div class="tw-thumbnail-image">
                    <img src=format!("/project_data_images/{}", project_context.image_url) alt="Screenshot of the project" />
                </div>

                <div class="tw-thumbnail-hover">
                    <span class="tw-thumbnail-hover-icon">
                        <Icon icon=IconSet::Eye />
                    </span>
                </div>
            </div>
        </A>
    }
}
