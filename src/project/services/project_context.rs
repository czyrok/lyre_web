#[cfg(feature = "ssr")]
use crate::project::{
    data::project_context::ProjectContext,
    repositories::project_context::ProjectContextRepository,
};

#[derive(Clone, Debug)]
pub struct ProjectContextService {
    #[cfg(feature = "ssr")]
    project_context_repository: ProjectContextRepository,
}

impl ProjectContextService {
    #[cfg(feature = "ssr")]
    pub fn new(project_context_repository: ProjectContextRepository) -> Self {
        Self {
            project_context_repository,
        }
    }

    // TODO: ajouter test sur la pagination (mettre de la merde) + voir si fonctione sans slug after
    #[cfg(feature = "ssr")]
    pub async fn get_ordered_project_contexts(
        &self,
        pagination_limit: u32,
        slug_cursor_after: Option<String>,
    ) -> Result<Vec<ProjectContext>, sqlx::Error> {
        let ordered_projects = self
            .project_context_repository
            .get_project_contexts(pagination_limit, slug_cursor_after)
            .await?;

        Ok(ordered_projects)
    }

    #[cfg(feature = "ssr")]
    /**
     * It returns the relevant projects' contexts
     * We consider the firsts as the relevant projects
     */
    pub async fn get_relevant_project_contexts(
        &self,
        limit: u32,
    ) -> Result<Vec<ProjectContext>, sqlx::Error> {
        self.get_ordered_project_contexts(limit, None).await
    }
}

#[cfg(feature = "ssr")]
impl Default for ProjectContextService {
    fn default() -> Self {
        ProjectContextService::new(ProjectContextRepository::default())
    }
}
