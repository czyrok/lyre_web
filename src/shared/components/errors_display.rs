use leptos::prelude::*;

use crate::core::data::fetch_state::FetchState;

#[component]
pub fn ErrorsDisplay(errors: ArcRwSignal<Errors>) -> impl IntoView {
    view! {
        {move || {
            errors
                .get()
                .into_iter()
                .map(|(_, error)| view! {
                    <span class="tw-error-info">{error.to_string()}</span>
                })
                .collect::<Vec<_>>()
        }}
    }
}
