use leptos::prelude::*;
use leptos::{config::LeptosOptions, IntoView};
use leptos_meta::MetaTags;
use leptos_meta::*;

use crate::app::App;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="fr">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
                <Meta name="color-scheme" content="dark light"/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}
