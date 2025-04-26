use leptos::prelude::OnceResource;

use crate::{
    core::data::fetch_state::FetchState,
    project::{
        api::project_tag_api::get_all_project_tags,
        dto::project_tags_dto::ProjectTagsDto,
    },
    shared::select::types::select_choice::SelectChoice,
};

#[derive(Clone, Copy)]
pub struct AllProjectTagsResource(
    OnceResource<
        Result<
            ProjectTagsDto,
            leptos::prelude::ServerFnError<
                crate::core::error::server_error_dto::ServerErrorDto,
            >,
        >,
    >,
);

impl AllProjectTagsResource {
    pub async fn get_select_choices(
        &self,
    ) -> Result<Vec<SelectChoice<String>>, FetchState> {
        self.0
            .await
            .map(|project_tags_dto| {
                project_tags_dto
                    .project_tags
                    .0
                    .iter()
                    .map(|tag| {
                        SelectChoice::new(
                            tag.name.clone(),
                            tag.name.clone(),
                            None,
                        )
                    })
                    .collect()
            })
            .map_err(FetchState::Errored)
    }
}

impl Default for AllProjectTagsResource {
    fn default() -> Self {
        let resource = OnceResource::new(get_all_project_tags());

        Self(resource)
    }
}
