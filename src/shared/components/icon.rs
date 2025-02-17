use leptos::prelude::*;

use crate::core::icon_set::IconSet;

#[component]
pub fn Icon(icon: IconSet) -> impl IntoView {
    match icon {
        IconSet::Check => view! {
            <svg xmlns="http://www.w3.org/2000/svg" width="100%" height="100%" fill="none" viewBox="0 0 60 60">
                <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5"
                    d="m7.5 32.443 8.84 10.125 22.098-25.313m14.062.175L28.392 42.744l-1.204-1.582" />
            </svg>
        },
    }
}
