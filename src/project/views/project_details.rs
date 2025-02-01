use leptos::{prelude::*, Params};
use leptos_router::{hooks::use_params, params::Params};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::project::api::project::get_project;

#[derive(Params, Clone, Debug, PartialEq, Eq)]
struct ProjectDetailsParams {
    slug: Option<String>,
}

// TODO: on garde ? Je pense remplacé par un ErrorViewState + qui utilise i18n
#[derive(Serialize, Deserialize, Error, Clone, PartialEq, Eq, Debug)]
pub enum ProjectDetailsError {
    #[error("Invalid project slug")]
    InvalidSlug,
    #[error("Server error: {0}.")]
    ServerError(String),
}

#[component]
pub fn ProjectDetails() -> impl IntoView {
    let query = use_params::<ProjectDetailsParams>();
    let slug = move || {
        query
            .get()
            .map(|q| q.slug.unwrap_or_default())
            .map_err(|_| ProjectDetailsError::InvalidSlug)
    };

    let project_resource = Resource::new_blocking(slug, |slug| async move {
        match slug {
            Err(e) => Err(e),
            Ok(slug) => get_project(slug)
                .await
                .map_err(|e| ProjectDetailsError::ServerError(e.to_string())),
        }
    });

    let project_view = move || {
        Suspend::new(async move {
            match project_resource.await {
                Ok(project_dto) => {
                    Ok(view! {
                        <h1>{project_dto.project.context.title}</h1>
                        <h1>{project_dto.project.content.0}</h1>
                        // <p>{post.content.clone()}</p>

                        // // since we're using async rendering for this page,
                        // // this metadata should be included in the actual HTML <head>
                        // // when it's first served
                        // <Title text=post.title/>
                        // <Meta name="description" content=post.content/>
                    })
                }
                Err(error) => {
                    Err(ProjectDetailsError::ServerError(error.to_string()))
                }
            }
        })
    };

    view! {
        <em>"The world's best content."</em>
        <Suspense fallback=move || view! { <p>"Loading post..."</p> }>
            <ErrorBoundary fallback=|errors| {
                #[cfg(feature = "ssr")]
                expect_context::<leptos_axum::ResponseOptions>()
                    .set_status(http::StatusCode::NOT_FOUND);
                view! {
                    <div class="error">
                        <h1>"Something went wrong."</h1>
                        <ul>
                            {move || {
                                errors
                                    .get()
                                    .into_iter()
                                    .map(|(_, error)| view! { <li>{error.to_string()}</li> })
                                    .collect::<Vec<_>>()
                            }}

                        </ul>
                    </div>
                }
            }>{project_view}</ErrorBoundary>
        </Suspense>
    }
}
