use leptos::prelude::*;

use crate::{
    core::{
        data::fetch_state::FetchState,
        error::frontend_error_type::FrontedErrorType,
    },
    shared::layouts::{
        info_error_page_layout::InfoErrorPageLayout,
        not_found_error_page_layout::NotFoundErrorPageLayout,
    },
};

#[component]
pub fn ProjectDetailsErrorDisplay(
    errors: ArcRwSignal<Errors>,
) -> impl IntoView {
    let mut is_project_not_found_error = false;

    let mut errors_count = 0;
    let mut single_error: Option<Error> = None;

    for error in errors.get().into_iter() {
        errors_count += 1;

        single_error = Some(error.1);
    }

    if errors_count != 1 {
        single_error = None;
    }

    if let Some(single_error) = single_error {
        if let Some(server_error_dto) =
            FetchState::get_error_dto_from(single_error)
        {
            if let FrontedErrorType::ProjectNotFound =
                server_error_dto.error_type
            {
                is_project_not_found_error = true;
            }
        }
    }

    let mut view = view! {
        <InfoErrorPageLayout
            content_renderer=move || {
                let errors = errors.clone();

                view! {
                    <h1 class="tw-title-size-lg">"Page d'erreur"</h1>

                    <div class="tw-info-error-page-layout-errors">
                        {move || {
                            errors
                                .get()
                                .into_iter()
                                .map(|(_, error)| view! {
                                    <span class="tw-error-info">{error.to_string()}</span>
                                })
                                .collect::<Vec<_>>()
                        }}
                    </div>
                }.into_any()
            }
        />
    }.into_any();

    if is_project_not_found_error {
        view = view! {
            <NotFoundErrorPageLayout title="Projet non trouvé" />
        }
        .into_any();
    }

    view
}
