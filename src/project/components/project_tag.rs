use leptos::prelude::*;

#[component]
pub fn ProjectTag(text: String) -> impl IntoView {
    view! {
        <span class="tw-project-tag">{ text }</span>
    }
}
