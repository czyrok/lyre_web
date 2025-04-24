#[cfg(feature = "ssr")]
use leptos::nonce::use_nonce;
use leptos::{config::LeptosOptions, prelude::*, IntoView};
use leptos_meta::*;

use crate::app::App;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    #[cfg(feature = "ssr")]
    let nonce: Option<String> = use_nonce().map(|nonce| nonce.to_string());
    #[cfg(not(feature = "ssr"))]
    let nonce = None::<String>;

    view! {
        <!DOCTYPE html>
        <html lang="fr">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>

                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>

                //// It's only useful when `@media (prefers-color-scheme: dark)` is used
                // <Meta name="color-scheme" content="dark light"/>

                // TODO: favicon
                //<Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
                <Link id="theme" rel="stylesheet" href="/tailwind_output.css" crossorigin=nonce.clone().unwrap_or_default() />
                <Link rel="preload" href="/fonts/GeistMono-1.3.0/variable-woff/GeistMonoVF.woff" as_="font" type_="font/woff2" crossorigin=nonce.clone().unwrap_or_default() />
                <Link rel="preload" href="/fonts/Geist-1.3.0/variable-woff/GeistVF.woff" as_="font" type_="font/woff2" crossorigin=nonce.clone().unwrap_or_default() />
                <Link rel="preload" href="/backgrounds/button_rectangle_background.svg" as_="image" type_="image/svg+xml" />
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}
