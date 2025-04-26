use leptos::prelude::*;

use crate::{
    project::data::next_project::NextProject,
    shared::components::pagination::next_pagination::NextPagination,
};

#[component]
pub fn ProjectDetailsPaginator(
    next_project: Option<NextProject>,
) -> impl IntoView {
    let view: AnyView = next_project.map_or(
        view! { "" }.into_any(),
        |next| {
            view! {
                <div class="tw-project-details-page-bottom-part">
                    <NextPagination click_text=next.title.clone().expect("`title` should exist") href=format!("/projects/{}/", next.slug) />
                </div>
            }.into_any()
        }
    );

    view
}
