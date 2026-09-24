use leptos::prelude::*;
use leptos_meta::*;

use crate::shared::{
    components::link::Link, enums::component_size::ComponentSize,
    layouts::secondary_page_layout::SecondaryPageLayout,
};

#[component]
pub fn LegalNotice() -> impl IntoView {
    view! {
        <Title text="Mentions Légales | Dylan Valentin" />

        <Meta name="author" content="Dylan Valentin" />
        <Meta name="description" content="Mentions légales du site de Dylan Valentin, développeur fullstack indépendant : éditeur, hébergement, propriété intellectuelle et données personnelles." />

        <Meta property="og:title" content="Mentions Légales | Dylan Valentin" />
        <Meta property="og:description" content="Mentions légales du site de Dylan Valentin, développeur fullstack indépendant." />

        <SecondaryPageLayout
            content_renderer=move || view! {
                <div class="legal-notice-page-top-part">
                    <Link size=ComponentSize::SM text="Accueil/" href="/" />

                    <h1 class="title-size-lg">"Mentions Légales"</h1>
                </div>

                <div class="legal-notice-page-middle-part">
                    <div class="middle-part-text">
                        <section class="text-section">
                            <h2>"Éditeur du site"</h2>
                            <p>
                                "Dylan Valentin — Entrepreneur individuel (EI)"<br/>
                                "5 avenue de la Gare, 69270 Fontaines-sur-Saône, France"<br/>
                                "Courriel : pro@dylan-valentin.dev"<br/>
                                "Téléphone : 07 83 39 50 56"<br/>
                                "Activité : prestation de services en développement informatique."<br/>
                                "SIREN : 977621119"<br/>
                                "SIRET : 97762111900014"<br/>
                                "TVA : TVA non applicable, article 293 B du Code général des impôts."
                            </p>
                        </section>

                        <section class="text-section">
                            <h2>"Directeur de la publication"</h2>
                            <p>"Dylan Valentin, joignable à pro@dylan-valentin.dev."</p>
                        </section>

                        <section class="text-section">
                            <h2>"Hébergement"</h2>
                            <p>
                                "Le site est auto-hébergé par son éditeur."<br/>
                                "Dylan Valentin — 1 avenue Rosa Parks, 69009 Lyon, France — 07 83 39 50 56."
                            </p>
                        </section>

                        <section class="text-section">
                            <h2>"Propriété intellectuelle"</h2>
                            <p>"Les textes, la charte graphique et le code de ce site sont la propriété de Dylan Valentin. Toute reproduction ou représentation, totale ou partielle, sans autorisation écrite préalable est interdite."</p>
                            <p>"Les icônes proviennent de SVG Repo (svgrepo.com) et restent soumises aux licences indiquées par leurs auteurs."</p>
                            <p>"Les polices Geist et Geist Mono sont éditées par Vercel et distribuées sous licence SIL Open Font License 1.1."</p>
                            <p>"Les captures d'écran illustrant les projets peuvent reproduire des interfaces, marques et logos appartenant à leurs titulaires respectifs. Elles sont publiées à titre d'illustration du travail réalisé, sans lien de partenariat ni approbation de leur part."</p>
                        </section>

                        <section class="text-section">
                            <h2>"Données personnelles"</h2>
                            <p>"Le site ne comporte ni formulaire, ni compte utilisateur. En dehors de la mesure d'audience décrite ci-dessous, aucune donnée personnelle n'est collectée lors de la navigation."</p>
                            <p>"Les messages adressés à pro@dylan-valentin.dev sont conservés le temps nécessaire au traitement de la demande. Vous disposez d'un droit d'accès, de rectification, d'effacement et d'opposition, exerçable à cette même adresse."</p>
                        </section>

                        <section class="text-section">
                            <h2>"Mesure d'audience"</h2>
                            <p>"La fréquentation du site est mesurée avec Umami, un logiciel libre installé et exploité par l'éditeur sur sa propre infrastructure, à l'adresse analytics.czrk.dev. Aucune donnée n'est transmise à un tiers ni hébergée hors de France, et aucun profilage publicitaire n'est réalisé."</p>
                            <p>"La mesure fonctionne sans cookie et sans identifiant persistant. Pour chaque page consultée sont enregistrés la page, son titre, le site référent, la langue et le navigateur, le système d'exploitation, le type d'appareil, la taille de l'écran ainsi qu'une localisation approximative déduite de l'adresse IP. Votre adresse IP n'est jamais enregistrée : elle sert uniquement, avec les informations de votre navigateur, à calculer un identifiant haché dont la clé change chaque jour, ce qui empêche tout rapprochement d'un jour sur l'autre."</p>
                            <p>"Ce traitement ne lit ni n'écrit aucune information sur votre terminal et se limite à des statistiques de fréquentation nécessaires à l'amélioration du site : il relève de l'intérêt légitime de l'éditeur et ne requiert pas votre consentement. Vous pouvez néanmoins vous y opposer à l'adresse pro@dylan-valentin.dev."</p>
                        </section>

                        <section class="text-section">
                            <h2>"Cookies"</h2>
                            <p>"Le site ne dépose aucun cookie. Le thème et la langue que vous choisissez sont enregistrés dans le stockage local de votre navigateur, à seule fin de réafficher le site tel que vous l'avez réglé. Ce stockage est strictement nécessaire au service et ne requiert pas votre consentement."</p>
                        </section>

                        <section class="text-section">
                            <h2>"Liens externes"</h2>
                            <p>"Ce site renvoie vers des sites tiers (dépôts de code, réseaux sociaux). Leur contenu n'engage que leurs éditeurs respectifs."</p>
                        </section>

                        <span class="additional-info">"Dernière mise à jour : 24 septembre 2026."</span>
                    </div>
                </div>
            }.into_any()
        />
    }
}
