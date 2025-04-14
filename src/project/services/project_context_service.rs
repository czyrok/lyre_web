use crate::project::{
    data::project_context::ProjectContext,
    repositories::{
        project_context_repository::ProjectContextRepository,
        project_repository::ProjectRepository,
    },
};

#[derive(Clone, Debug)]
pub struct ProjectContextService {
    project_context_repository: ProjectContextRepository,
    project_repository: ProjectRepository,
}

impl ProjectContextService {
    pub fn new(
        project_context_repository: ProjectContextRepository,
        project_repository: ProjectRepository,
    ) -> Self {
        Self {
            project_context_repository,
            project_repository,
        }
    }

    // TODO: ajouter test sur la pagination (mettre de la merde) + voir si fonctione sans slug after
    pub async fn get_ordered_project_contexts(
        &self,
        pagination_limit: u32,
        slug_cursor_after: Option<String>,
    ) -> Result<(Vec<ProjectContext>, u32), sqlx::Error> {
        let ordered_projects = self
            .project_context_repository
            .get_project_contexts(pagination_limit, slug_cursor_after.clone())
            .await?;

        let project_total_count =
            self.project_repository.get_project_total_count().await?;

        Ok((ordered_projects, project_total_count))
    }

    /**
     * It returns the relevant projects' contexts
     * We consider the firsts as the relevant projects
     */
    pub async fn get_relevant_project_contexts(
        &self,
        limit: u32,
    ) -> Result<Vec<ProjectContext>, sqlx::Error> {
        let ordered_projects = self
            .project_context_repository
            .get_project_contexts(limit, None)
            .await?;

        Ok(ordered_projects)
    }
}
