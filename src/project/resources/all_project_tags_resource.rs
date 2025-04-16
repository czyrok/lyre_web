use leptos::prelude::OnceResource;

use crate::{
    project::{
        api::project_tag_api::get_all_project_tags,
        dto::project_tags_dto::ProjectTagsDto,
    },
    shared::select::types::select_choice::SelectChoice,
};

#[derive(Clone, Copy)]
pub struct AllProjectTagsResource(
    pub  OnceResource<
        Result<
            ProjectTagsDto,
            leptos::prelude::ServerFnError<
                crate::core::error::server_error_dto::ServerErrorDto,
            >,
        >,
    >,
);

impl AllProjectTagsResource {
    pub async fn get_select_choices(&self) -> Vec<SelectChoice<String>> {
        let project_tags = self.0.await.unwrap_or_default().project_tags.0;

        project_tags
            .iter()
            .map(|tag| {
                SelectChoice::new(tag.name.clone(), tag.name.clone(), None)
            })
            .collect()
    }
}

impl Default for AllProjectTagsResource {
    fn default() -> Self {
        let resource = OnceResource::new(get_all_project_tags());

        Self(resource)
    }
}
