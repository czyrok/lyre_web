use leptos::prelude::*;

use crate::{
    core::data::fetch_state::FetchState,
    project::data::project_context::ProjectContext,
    shared::components::fetch_error_display::FetchErrorDisplay,
};

#[component]
pub fn SearchResultInfo(
    project_contexts: Signal<Vec<ProjectContext>>,
    last_fetch_state: Signal<FetchState>,
) -> impl IntoView {
    view! {
        <div class="tw-middle-part-info">
            <Show
                when=move || { last_fetch_state.get().is_ok() && project_contexts.get().is_empty() }
            >
                <span class="tw-additional-info">
                    "Aucun résultat."
                </span>
            </Show>

            <FetchErrorDisplay fetch_state=last_fetch_state />
        </div>
    }
}
