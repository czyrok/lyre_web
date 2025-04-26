use leptos::prelude::*;

use crate::core::data::icon_set::IconSet;

#[component]
pub fn Icon(icon: IconSet) -> impl IntoView {
    match icon {
        IconSet::Check => view! {
            <svg xmlns="http://www.w3.org/2000/svg" width="100%" height="100%" fill="none" viewBox="0 0 60 60">
                <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5"
                    d="m7.5 32.443 8.84 10.125 22.098-25.313m14.062.175L28.392 42.744l-1.204-1.582" />
            </svg>
        }.into_any(),
        IconSet::External => view! {
            <svg width="100%" height="100%" viewBox="0 0 60 60" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path
                    d="M22.5006 10.5H17.1006C13.7403 10.5 12.0589 10.5 10.7754 11.154C9.64641 11.7292 8.72919 12.6464 8.15397 13.7754C7.5 15.0589 7.5 16.7403 7.5 20.1006V42.9006C7.5 46.2609 7.5 47.9403 8.15397 49.2237C8.72919 50.3526 9.64641 51.2715 10.7754 51.8466C12.0576 52.5 13.737 52.5 17.0907 52.5H39.9093C43.263 52.5 44.94 52.5 46.2222 51.8466C47.3511 51.2715 48.2715 50.3517 48.8466 49.2228C49.5 47.9406 49.5 46.263 49.5 42.9093V37.5M52.5 22.5V7.5M52.5 7.5H37.5M52.5 7.5L31.5 28.5"
                    stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" />
            </svg>
        }.into_any(),
        IconSet::Search => view! {
            <svg width="100%" height="100%" viewBox="0 0 60 60" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path
                    d="M27.4545 14.6232C34.358 14.6232 39.9545 20.2196 39.9545 27.1232M41.6015 41.2604L52.4545 52.1232M47.4545 27.1232C47.4545 38.1689 38.5003 47.1232 27.4545 47.1232C16.4088 47.1232 7.45453 38.1689 7.45453 27.1232C7.45453 16.0775 16.4088 7.12317 27.4545 7.12317C38.5003 7.12317 47.4545 16.0775 47.4545 27.1232Z"
                    stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" />
            </svg>
        }.into_any(),
        IconSet::RightArrow => view! {
            <svg xmlns="http://www.w3.org/2000/svg" width="100%" height="100%" fill="none" viewBox="0 0 60 60">
                <path stroke="currentColor" stroke-linecap="round" stroke-width="1.5" d="M52.059 30.001H7.5m45 0-15.441-15.44M52.5 30.001 37.059 45.442"/>
            </svg>
        }.into_any(),
        IconSet::Eye => view! {
            <svg width="100%" height="100%" viewBox="0 0 60 60" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M7.5 29.9986C7.5 29.9986 15.6818 13.635 30 13.635C44.3182 13.635 52.5 29.9986 52.5 29.9986"
                    stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" />
                <path
                    d="M7.5 29.9987C7.5 29.9987 15.6818 46.3623 30 46.3623C44.3182 46.3623 52.5 29.9987 52.5 29.9987"
                    stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" />
                <path
                    d="M30 36.135C33.389 36.135 36.1364 33.3877 36.1364 29.9987C36.1364 26.6096 33.389 23.8623 30 23.8623C26.611 23.8623 23.8636 26.6096 23.8636 29.9987C23.8636 33.3877 26.611 36.135 30 36.135Z"
                    stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" />
            </svg>
        }.into_any(),
        IconSet::Compass => view! {
            <svg xmlns="http://www.w3.org/2000/svg" width="100%" height="100%" fill="none" viewBox="0 0 60 60">
                <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M30 30h.025M52.5 30c0 12.426-10.074 22.5-22.5 22.5S7.5 42.426 7.5 30 17.574 7.5 30 7.5 52.5 17.574 52.5 30ZM40 20l-16.25 3.75L20 40l16.25-3.75L40 20Z"/>
            </svg>
        }.into_any(),
        IconSet::LinkedIn => view! {
            <svg xmlns="http://www.w3.org/2000/svg" width="100%" height="100%" fill="none" viewBox="0 0 60 60">
                <path stroke="currentColor" stroke-linecap="round" stroke-width="1.5" d="M32.261 22.446h-8.31V52.5h8.606V38.73c0-3.652.689-7.162 5.193-7.162s4.513 4.15 4.513 7.392V52.5h8.606V37.133c0-7.42-1.578-14.267-10.337-14.267-3.232-.124-6.358.47-8.099 3.108a.105.105 0 0 1-.2-.057l.028-3.471Zm-22.395 0h8.683V52.5H9.866V22.446ZM14.188 7.5a5.04 5.04 0 1 0 5.02 5.059 5.02 5.02 0 0 0-5.02-5.059Z"/>
            </svg>
        }.into_any(),
        IconSet::SingleDownArrow => view! {
            <svg xmlns="http://www.w3.org/2000/svg" width="100%" height="100%" fill="none" viewBox="0 0 60 60">
                <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M7.5 22.75 30 45.25l22.5-22.5"/>
            </svg>
        }.into_any(),
        IconSet::DoubleDownArrow => view! {
            <svg xmlns="http://www.w3.org/2000/svg" width="100%" height="100%" fill="none" viewBox="0 0 60 60">
                <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M11.25 33.75 30 52.5l18.75-18.75M11.25 7.5 30 26.25 48.75 7.5"/>
            </svg>
        }.into_any(),
        IconSet::Home => view! {
            <svg xmlns="http://www.w3.org/2000/svg" width="100%" height="100%" fill="none" viewBox="0 0 60 60">
                <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="m11.174 16.314 15.862-8.097a6.493 6.493 0 0 1 5.93 0l15.863 8.097a7.492 7.492 0 0 1 3.575 7.238l.096 16.09c0 7.102-5.757 12.858-12.857 12.858H20.357c-7.1 0-12.857-5.756-12.857-12.857l.1-16.072a7.493 7.493 0 0 1 3.574-7.258Z" clip-rule="evenodd"/>
                <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M22.504 52.5v-7.502a7.502 7.502 0 0 1 15.005 0V52.5"/>
            </svg>
        }.into_any(),
        IconSet::About => view! {
            <svg xmlns="http://www.w3.org/2000/svg" width="100%" height="100%" fill="none" viewBox="0 0 60 60">
                <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M30.125 20h-.25M30 32.5V40M26.35 7.775A22.5 22.5 0 0 0 10 40.275L8.25 48.8a2.5 2.5 0 0 0 2.95 2.95L19.675 50a22.501 22.501 0 0 0 31.669-26.993A22.5 22.5 0 0 0 26.35 7.775Z"/>
            </svg>
        }.into_any(),
        IconSet::Email => view! {
            <svg xmlns="http://www.w3.org/2000/svg" width="100%" height="100%" fill="none" viewBox="0 0 60 60">
                <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M40 30c0 5.523-4.477 10-10 10s-10-4.477-10-10 4.477-10 10-10 10 4.477 10 10Zm0 0v3.75a6.25 6.25 0 0 0 12.5 0V30C52.5 17.574 42.426 7.5 30 7.5S7.5 17.574 7.5 30 17.574 52.5 30 52.5h10"/>
            </svg>
        }.into_any(),
        IconSet::Undo => view! {
            <svg xmlns="http://www.w3.org/2000/svg" width="100%" height="100%" fill="none" viewBox="0 0 60 60">
                <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M7.5 20.157h30.938c7.766 0 14.062 6.296 14.062 14.063 0 7.766-6.296 14.063-14.063 14.063H18.75M7.5 20.156l8.438-8.437M7.5 20.157l8.438 8.438"/>
            </svg>
        }.into_any(),
        IconSet::Calendar => view! {
            <svg xmlns="http://www.w3.org/2000/svg" width="100%" height="100%" fill="none" viewBox="0 0 60 60">
                <path stroke="currentColor" stroke-linecap="round" stroke-width="1.5" d="M7.5 22.5h45m-35-15v5m25-5v5M15 30h5m7.5 0h5m7.5 0h5m-30 7.5h5m7.5 0h5m7.5 0h5M15 45h5m7.5 0h5m7.5 0h5m-29.5 7.5h29c2.8 0 4.2 0 5.27-.545a5 5 0 0 0 2.185-2.185c.545-1.07.545-2.47.545-5.27v-24c0-2.8 0-4.2-.545-5.27a4.999 4.999 0 0 0-2.185-2.185C48.7 12.5 47.3 12.5 44.5 12.5h-29c-2.8 0-4.2 0-5.27.545a4.999 4.999 0 0 0-2.185 2.185C7.5 16.3 7.5 17.7 7.5 20.5v24c0 2.8 0 4.2.545 5.27a5 5 0 0 0 2.185 2.185c1.07.545 2.47.545 5.27.545Z"/>
            </svg>
        }.into_any(),
        IconSet::Github => view! {
            <svg xmlns="http://www.w3.org/2000/svg" width="100%" height="100%" fill="none" viewBox="0 0 60 60">
                <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M22.895 52.281v-6.398l.084-4.547a6.424 6.424 0 0 1 1.599-3.825.065.065 0 0 0-.04-.109c-6.059-.756-12.372-3.314-12.372-14.452-.039-2.744.856-5.403 2.51-7.489.058-.073.117-.145.177-.216a.359.359 0 0 0 .067-.336l-.006-.02a11.328 11.328 0 0 1 .284-7.41.249.249 0 0 1 .185-.16c.537-.096 2.708-.225 6.865 2.646.13.089.26.181.394.277.096.068.218.09.332.057l.01-.003a24.366 24.366 0 0 1 13.327-.01l.014.004a.458.458 0 0 0 .39-.068c.125-.088.247-.174.368-.257 4.147-2.865 6.306-2.743 6.846-2.647a.255.255 0 0 1 .189.162l.012.032a11.269 11.269 0 0 1 .283 7.387.38.38 0 0 0 .072.354l.008.01c.054.065.108.131.161.198 1.654 2.086 2.55 4.745 2.51 7.489 0 11.198-6.358 13.69-12.445 14.369a.043.043 0 0 0-.028.072l.028.03a7.004 7.004 0 0 1 1.399 2.386c.322.93.454 1.923.385 2.912v9.562"/>
                <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9.665 42.281a6.131 6.131 0 0 1 4.46 3.02 6.25 6.25 0 0 0 3.738 2.99 6.208 6.208 0 0 0 4.75-.546"/>
            </svg>
        }.into_any()
    }
}
