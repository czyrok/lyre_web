use crate::project::repositories::project_slug_repository::ProjectSlugRepository;

#[derive(Clone, Debug)]
pub struct ProjectSlugService {
    project_slug_repository: ProjectSlugRepository,
}

impl ProjectSlugService {
    pub fn new(project_slug_repository: ProjectSlugRepository) -> Self {
        Self {
            project_slug_repository,
        }
    }

    pub async fn get_project_slugs(&self) -> Result<Vec<String>, sqlx::Error> {
        self.project_slug_repository.get_project_slugs().await
    }
}
