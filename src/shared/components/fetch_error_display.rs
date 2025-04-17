use leptos::prelude::*;

use crate::{
    core::error::server_function_error::ServerFunctionException,
    project::data::project_context::ProjectContext,
};

#[component]
pub fn FetchErrorDisplay(
    fetch_error: Signal<Result<(), ServerFunctionException>>,
) -> impl IntoView {
    view! {
        <Show when=move || { fetch_error.get().is_err() }>
            <span class="tw-error-info">{ fetch_error.get().unwrap_err().to_string() }</span>
        </Show>
    }
}
