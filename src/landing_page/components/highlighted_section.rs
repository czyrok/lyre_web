use leptos::prelude::*;

use crate::shared::{
    components::link::Link, enums::component_size::ComponentSize,
};

#[component]
pub fn HighlightedSection() -> impl IntoView {
    view! {
        <div class="tw-landing-page-section-container tw-landing-page-highlighted-section">
            <div id="about" class="tw-anchor"></div>

            <div class="tw-section-text">
                <h1 class="tw-title-size-xl">"À Propos"</h1>

                <div class="tw-section-text-timeline">
                    <div class="tw-timeline-item">
                        <span class="tw-item-date">2020</span>

                        <p>
                            "Tout a commencé lors du 1er confinement, par ennui j'ai commencé la "
                            <b>"programmation"</b>
                            " (j'avais déjà un pas dans ce domaine grâce à mon père). Cela s'est traduit par la création d'une application PC pour accéder à un jeu en ligne (un serveur privé Habbo). Cette application a fini par se transformer en navigateur web, "
                            <Link size=ComponentSize::MD text="Kepler" href="/project/kepler/" />
                            " qui a connu de nombreuses mises à jour pendant 1 an. Après mon Bac, j'ai décidé de continuer mes études dans l'informatique."
                        </p>
                    </div>

                    <div class="tw-timeline-item">
                        <span class="tw-item-date">2021-2024</span>

                        <p>
                            "J'ai donc intégré l'"
                            <b>"IUT Lyon 1"</b>
                            " en BUT Informatique où j'ai pu me spécialiser davantage dans ce domaine. Au fil de mes années en BUT, j'ai réalisé plusieurs projets perso. mais aussi des projets tuteurés (dans le cadre de ma formation) qui m'ont permis d'acquérir de nouvelles compétences. En 2023, une occasion de job d'été en tant qu'indépendant s'est présentée, c'est à ce moment-là que j'ai pu découvrir le freelance (j'étais alors en 2e année)."
                        </p>
                    </div>

                    <div class="tw-timeline-item">
                        <span class="tw-item-date">2024</span>

                        <p>
                            "À la suite de mon BUT j'ai décidé d'arrêter mes études pour me consacrer à temps plein au métier de freelance. Ce métier me permet une grande "
                            <b>"flexibilité"</b>
                            " mais aussi l'occasion de voir toutes sortes de projets différents grâce à mes clients. Par ailleurs, ça me permet de mon côté d'avancer sur des projets perso. qui pourront sortir sous le pavillon de mon entreprise. Aujourd'hui, je travaille pour "
                            <b>"Ubikap"</b>
                            " en freelance qui était mon entreprise d'alternance en 3e année."
                        </p>
                    </div>
                </div>
            </div>
        </div>
    }
}
