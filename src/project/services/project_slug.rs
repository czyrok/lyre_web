#[cfg(feature = "ssr")]
use crate::project::repositories::project_slug::ProjectSlugRepository;

#[derive(Clone, Debug)]
pub struct ProjectSlugService {
    #[cfg(feature = "ssr")]
    project_slug_repository: ProjectSlugRepository,
}

impl ProjectSlugService {
    #[cfg(feature = "ssr")]
    pub fn new(project_slug_repository: ProjectSlugRepository) -> Self {
        Self {
            project_slug_repository,
        }
    }

    #[cfg(feature = "ssr")]
    pub async fn get_project_slugs(&self) -> Result<Vec<String>, sqlx::Error> {
        let project_slugs =
            self.project_slug_repository.get_project_slugs().await?;

        Ok(project_slugs)
    }
}
