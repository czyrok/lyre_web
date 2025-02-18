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
    }
}
