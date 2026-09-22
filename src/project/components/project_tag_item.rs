use leptos::prelude::*;

#[component]
pub fn ProjectTagItem(text: String) -> impl IntoView {
    view! {
        <span class="project-tag">{ text }</span>
    }
}
