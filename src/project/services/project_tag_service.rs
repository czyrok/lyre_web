use crate::project::{
    data::project_tags::ProjectTags,
    repositories::project_tag_repository::ProjectTagRepository,
};

#[derive(Clone, Debug)]
pub struct ProjectTagService {
    project_tag_repository: ProjectTagRepository,
}

impl ProjectTagService {
    pub fn new(project_tag_repository: ProjectTagRepository) -> Self {
        Self {
            project_tag_repository,
        }
    }

    pub async fn get_all_project_tags(
        &self,
    ) -> Result<ProjectTags, sqlx::Error> {
        self.project_tag_repository.get_all_project_tags().await
    }
}
