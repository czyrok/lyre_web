use leptos::{config::LeptosOptions, prelude::*, IntoView};
use leptos_meta::{MetaTags, *};

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
                <Stylesheet id="theme" href="tailwind_output.css"/>
                // TODO:
                //<Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}
