use leptos::prelude::*;

use crate::project::{
    components::project_link_items::ProjectLinkItems,
    data::project_links::ProjectLinks,
};

#[component]
pub fn ProjectDetailsActions(project_links: ProjectLinks) -> impl IntoView {
    view! {
        {move || (!project_links.0.is_empty()).then(|| {
            view! {
                <div class="tw-top-part-actions">
                    <ProjectLinkItems project_links=project_links.clone() />
                </div>
            }
        })}
    }
}
