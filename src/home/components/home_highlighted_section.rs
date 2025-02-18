use leptos::prelude::*;

#[component]
pub fn HomeHighlightedSection() -> impl IntoView {
    view! {
        <div class="tw-home-section-container tw-home-highlighted-section">
            <div class="tw-section-text">
                <h1 class="tw-title-size-xl">"À Propos"</h1>

                <div class="tw-section-text-pair">
                    <p>"Tout a commencé lors du 1er confinement, par ennui j'ai commencé la programmation (j'avais déjà un pas dans ce domaine grâce à mon père). Cela s'est traduit par la création d'une application PC pour accéder à un jeu en ligne (un serveur privé Habbo). Cette application a fini par se transformer en navigateur web, Kepler. Après mon Bac, j'ai intégré l'IUT Lyon 1 en BUT Informatique où j'ai pu me spécialiser davantage dans ce domaine. Au fil de mes années en BUT, j'ai réalisé plusieurs projets perso. mais aussi des projets tuteurés (dans le cadre de ma formation) qui m'ont permis d'acquérir de nouvelles compétences."</p>

                    <p>"En 2023, une occasion de job d'été en tant qu'indépendant s'est présentée, c'est à ce moment-là que j'ai pu découvrir le freelance (j'étais alors en 2e année). Puis, à la suite de mon BUT j'ai décidé d'arrêter mes études pour me consacrer à temps plein au métier de freelance. Ce métier me permet une grande flexibilité mais aussi l'occasion de voir toutes sortes de projets différents grâce à mes clients. Par ailleurs, ça me permet de mon côté d'avancer sur des projets perso. qui pourront sortir sous le pavillon de mon entreprise. Aujourd'hui, je travaille pour Ubikap en freelance qui était mon entreprise d'alternance en 3e année."</p>
                </div>
            </div>
        </div>
    }
}
