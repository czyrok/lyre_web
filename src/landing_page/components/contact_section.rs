use leptos::prelude::*;

use crate::{
    core::data::icon_set::IconSet,
    shared::{
        button::{
            components::{
                primary_button_as_link::PrimaryButtonAsLink,
                secondary_button_as_link::SecondaryButtonAsLink,
            },
            types::icon_side::IconSide,
        },
        enums::component_size::ComponentSize,
    },
};

#[component]
pub fn ContactSection() -> impl IntoView {
    view! {
        //// Causes issue in static mode when I go to '/projects' from '/'
        //// The head of the document will be impacted, and, Tailwind output will not be loaded
        // <Link rel="preload" href="/images/ma_photo.png" as_="image" type_="image/png" />

        <div class="tw-landing-page-section-container tw-landing-page-contact-section">
            <div id="contact" class="tw-anchor"></div>

            <div class="tw-section-photo">
                <img width="100%" height="100%" src="/images/ma_photo.png" alt="Photo of the owner of the website" />
            </div>

            <div class="tw-section-right">
                <div class="tw-section-text">
                    <h1 class="tw-title-size-xl">"Contact"</h1>

                    <p>"Même si dans l'instant je suis occupé, ma porte est toujours ouverte pour discuter. Alors n'hésite pas !"</p>
                </div>

                <div class="tw-section-actions">
                    <PrimaryButtonAsLink size=ComponentSize::LG text="Par Mail" href="mailto:dylanvalentin2003@gmail.com" icon=IconSet::External icon_side=IconSide::Right/>
                    <SecondaryButtonAsLink size=ComponentSize::LG text="LinkedIn" href="https://www.linkedin.com/in/dylan-valentin/" icon=IconSet::LinkedIn icon_side=IconSide::Right target="_blank"/>
                </div>
            </div>
        </div>
    }
}
