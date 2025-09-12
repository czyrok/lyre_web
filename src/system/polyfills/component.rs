use leptos::{logging::error, prelude::*};
use leptos_router::hooks::use_url;

use super::css_anchor_positioning;

#[component]
pub fn PolyfillHandler() -> impl IntoView {
    #[cfg(not(feature = "ssr"))]
    {
        let current_url = use_url();

        Effect::new(move || {
            current_url.track();

            match css_anchor_positioning::apply_polyfill() {
                Err(_) => {
                    error!("Unable to load 'css_anchor_positioning' polyfill")
                }
                _ => (),
            }
        });
    }

    view! {}
}
