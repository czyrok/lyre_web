use leptos::prelude::*;

use crate::{
    core::{component_size::ComponentSize, icon_set::IconSet},
    shared::components::button::{
        icon_side::IconSide, primary_button_as_link::PrimaryButtonAsLink,
        secondary_button_as_link::SecondaryButtonAsLink,
    },
};

#[component]
pub fn ContactSection() -> impl IntoView {
    view! {
        <div class="tw-landing-page-section-container tw-landing-page-contact-section">
            <div id="contact" class="tw-anchor"></div>

            <div class="tw-section-photo">
                <img width="100%" height="100%" src="images/ma_photo.png" alt="Photo of the owner of the website" />
            </div>

            <div class="tw-section-right">
                <div class="tw-section-text">
                    <h1 class="tw-title-size-xl">"Contact"</h1>

                    <p>"Même si dans l'instant je suis occupé, ma porte est toujours ouverte pour discuter. Alors n'hésite pas !"</p>
                </div>

                <div class="tw-section-actions">
                    <PrimaryButtonAsLink size=ComponentSize::LG text="Par Mail".into() href="mailto:dylanvalentin2003@gmail.com".into() icon=IconSet::External icon_side=IconSide::Right/>
                    <SecondaryButtonAsLink size=ComponentSize::LG text="LinkedIn".into() href="https://www.linkedin.com/in/dylan-valentin/".into() icon=IconSet::LinkedIn icon_side=IconSide::Right target="_blank"/>
                </div>
            </div>
        </div>
    }
}
