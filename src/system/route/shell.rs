#[cfg(feature = "ssr")]
use leptos::nonce::use_nonce;
use leptos::{config::LeptosOptions, prelude::*, IntoView};
use leptos_meta::*;

use crate::app::App;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    // TODO: add nonce field when it will available on `Link` components
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

                // Preloading
                <Link id="theme" rel="stylesheet" href="/tailwind_output.css" crossorigin="" />
                <Link rel="preload" href="/fonts/GeistMono-1.3.0/variable-woff/GeistMonoVF.woff" as_="font" type_="font/woff2" crossorigin="" />
                <Link rel="preload" href="/fonts/Geist-1.3.0/variable-woff/GeistVF.woff" as_="font" type_="font/woff2" crossorigin="" />
                <Link rel="preload" href="/backgrounds/button_rectangle_background.svg" as_="image" type_="image/svg+xml" crossorigin="" />

                // Polyfills
                <script type="module" nonce=nonce>
                    "
                    //// Used only by Firefox and Safari to fix dropdown menu positioning
                    //// Source: https://github.com/oddbird/css-anchor-positioning
                    if (!('anchorName' in document.documentElement.style)) {
                        const { default: polyfill } = await import('/polyfills/@oddbird/debug-css-anchor-positioning-fn@0.6.1.js');

                        polyfill({
                            elements: undefined,
                            excludeInlineStyles: false,
                            useAnimationFrame: false,
                        });

                        const observeUrlChange = () => {
                            let oldHref = document.location.href;
                            const body = document.querySelector('body');
                            const observer = new MutationObserver(mutations => {
                                if (oldHref !== document.location.href) {
                                    oldHref = document.location.href;
                                    /* Changed ! your code here */

                                    // setTimeout(() => {
                                        const els = document.querySelectorAll('[data-generated-by-polyfill=\"true\"]');

                                        console.error(els.length);

                                        for (const el of els) {
                                            el.remove()
                                        }

                                        const cleanEvent = new Event('css-anchor-positioning-clean');
                                        window.dispatchEvent(cleanEvent);

                                        console.error('slt');
                                        setTimeout(() => {
                                            polyfill({
                                                elements: undefined,
                                                excludeInlineStyles: false,
                                                useAnimationFrame: false,
                                            });
                                        }, 100)

    //                                     const currentThemeStyle = document.getElementById('theme');

    //                                     if (currentThemeStyle) {
    //                                         const newThemeStyle = document.createElement('style');

    //                                         const themeStyleHref = currentThemeStyle.getAttribute('data-original-href')

    //                                         if (themeStyleHref) {
    //                                             console.error('href: data-original-href', themeStyleHref)

    //                                             fetch(themeStyleHref).then((res) => res.text()).then((themeStyle) => {
    //                                                 newThemeStyle.id = 'theme'
    //                                                 // newThemeStyle.rel = 'stylesheet'
    //                                                 newThemeStyle.setAttribute('rel', 'stylesheet')
    //                                                 // newThemeStyle.setAttribute('href', themeStyleHref)
    //                                                 newThemeStyle.textContent = themeStyle
    //                                                 newThemeStyle.setAttribute('data-original-href', themeStyleHref)

    //                                                 document.head.insertAdjacentElement('beforeend', newThemeStyle);

    //                                                 currentThemeStyle.remove();
    // =
    //                                                 for (const el of els) {
    //                                                     el.remove()
    //                                                 }

    //                                                 console.error('slt');
    //                                                 polyfill({
    //                                                     elements: undefined,
    //                                                     excludeInlineStyles: false,
    //                                                     useAnimationFrame: false,
    //                                                 });
    //                                             })

                                            
    //                                         } else {
    //                                             console.error('flute')
    //                                         }
    //                                     }

                                        
                                    // }, 3e3)

                                    
                                }
                            });
                            observer.observe(body, { childList: true, subtree: true });
                        };

                        window.onload = observeUrlChange;
                    }

                    //// Used only Safari to fix focus on buttons, links, checkboxes etc...
                    //// Source: https://itnext.io/fixing-focus-for-safari-b5916fef1064
                    import('/polyfills/@NickGuard/safari-focus@2.0.js');
                    "
                </script>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}
