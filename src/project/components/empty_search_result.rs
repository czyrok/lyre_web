use leptos::prelude::*;

use crate::project::data::project_context::ProjectContext;

#[component]
pub fn EmptySearchResult(
    project_contexts: Signal<Vec<ProjectContext>>,
) -> impl IntoView {
    view! {
        <Show
            when=move || { project_contexts.get().is_empty() }
        >
            <div class="tw-project-search-page-bottom-part">
                <div class="tw-pagination">
                    <span class="tw-pagination-text">
                        "Aucun résultat."
                    </span>
                </div>
            </div>
        </Show>
    }
}
