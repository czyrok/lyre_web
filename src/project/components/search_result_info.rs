use leptos::prelude::*;

use crate::{
    core::error::server_function_error::ServerFunctionException,
    project::data::project_context::ProjectContext,
    shared::components::fetch_error_display::FetchErrorDisplay,
};

#[component]
pub fn SearchResultInfo(
    project_contexts: Signal<Vec<ProjectContext>>,
    last_fetch_error: Signal<Result<(), ServerFunctionException>>,
) -> impl IntoView {
    view! {
        <div class="tw-middle-part-info">
            <Show
                when=move || { last_fetch_error.get().is_ok() && project_contexts.get().is_empty() }
            >
                <span class="tw-additional-info">
                    "Aucun résultat."
                </span>
            </Show>

            <FetchErrorDisplay fetch_error=last_fetch_error />
        </div>
    }
}
