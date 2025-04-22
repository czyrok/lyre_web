use leptos::prelude::*;

#[component]
pub fn ProjectTagItem(text: String) -> impl IntoView {
    view! {
        <span class="tw-project-tag">{ text }</span>
    }
}
