use chrono::Utc;
use leptos_router::static_routes::ResolvedStaticPath;
use sitemap_generator::{ChangeFreq, UrlEntry};

use super::{
    super::state::app_state::AppState, static_route_generator::get_static_paths,
};

// TODO: test sur le résultat
pub async fn resolve_sitemap_entries(
    app_state: AppState,
    patched_entries: Vec<UrlEntry>,
) -> Vec<UrlEntry> {
    let path_prefix = app_state.environment.full_website_url.clone();

    let static_paths = get_static_paths(app_state).await;

    let entries = static_path_into_url_entries(static_paths);
    let mut entries = patch_entries(entries, patched_entries);

    for entry in entries.iter_mut() {
        let href = entry.loc.clone();
        let full_href = if href.starts_with("/") {
            format!("{path_prefix}{href}")
        } else {
            format!("{path_prefix}/{href}")
        };

        entry.loc = full_href;
    }

    entries
}

fn patch_entries(
    entries: Vec<UrlEntry>,
    changed_entries: Vec<UrlEntry>,
) -> Vec<UrlEntry> {
    let mut new_entries = vec![];

    for entry in entries.iter() {
        let mut new_entry = entry.clone();

        for changed_entry in changed_entries.clone().into_iter() {
            if entry.loc == changed_entry.loc {
                new_entry = changed_entry;
            }
        }

        new_entries.push(new_entry);
    }

    new_entries
}

fn static_path_into_url_entries(
    paths: Vec<ResolvedStaticPath>,
) -> Vec<UrlEntry> {
    paths
        .iter()
        .map(|path| {
            let path_string: String = path.to_string();

            UrlEntry::new(path_string)
                .changefreq(ChangeFreq::Never)
                .priority(0.5)
                .lastmod(Utc::now().format("%Y-%m-%d").to_string())
        })
        .collect()
}
