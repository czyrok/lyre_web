use leptos::prelude::*;

use crate::{
    core::cursor_pagination::CursorPagination,
    project::api::project_context::get_ordered_project_contexts,
};

#[component]
pub fn LandingPage() -> impl IntoView {
    let resource = Resource::new(
        || (),
        |_| {
            get_ordered_project_contexts(CursorPagination {
                limit: 12,
                cursor_after: None,
            })
        },
    );

    let project_contexts = move || {
        resource
            .get()
            .map(|n| n.unwrap_or_default())
            .map(|project_contexts_dto| project_contexts_dto.project_contexts)
            .unwrap_or_default()
    };

    view! {
        <h1>"My Great Blog"</h1>
        <Suspense fallback=move || view! { <p>"Loading posts..."</p> }>
            <ul>
                <For each=project_contexts key=|project_context| project_context.slug.clone() let:project_context>
                    <li>
                        <a href=format!("/projects/{}/", project_context.slug)>{project_context.title.clone()}</a>
                    </li>
                </For>
            </ul>
        </Suspense>
    }
}
