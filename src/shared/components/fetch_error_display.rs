use leptos::prelude::*;

use crate::core::data::fetch_state::FetchState;

#[component]
pub fn FetchErrorDisplay(fetch_state: Signal<FetchState>) -> impl IntoView {
    view! {
        <Show when=move || { fetch_state.get().is_errored() }>
            <span class="tw-error-info">{ fetch_state.get().unwrap_error().to_string() }</span>
        </Show>
    }
}
