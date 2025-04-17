use leptos::prelude::*;

use crate::core::error::server_function_error::ServerFunctionException;

#[derive(Clone)]
pub struct FetchErrorState(Result<(), ServerFunctionException>);

impl FetchErrorState {
    pub fn errored(error: ServerFunctionException) -> Self {
        Self(Err(error))
    }

    pub fn is_ok(&self) -> bool {
        self.0.is_ok()
    }

    pub fn is_err(&self) -> bool {
        self.0.is_err()
    }

    pub fn unwrap_err(self) -> ServerFunctionException {
        self.0.unwrap_err()
    }
}

impl Default for FetchErrorState {
    fn default() -> Self {
        Self(Ok(()))
    }
}

#[component]
pub fn FetchErrorDisplay(
    fetch_error: Signal<FetchErrorState>,
) -> impl IntoView {
    view! {
        <Show when=move || { fetch_error.get().is_err() }>
            <span class="tw-error-info">{ fetch_error.get().unwrap_err().to_string() }</span>
        </Show>
    }
}
