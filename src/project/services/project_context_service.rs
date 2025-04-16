use crate::{
    core::dto::cursor_pagination_dto::CursorPaginationDto,
    project::{
        data::project_context::ProjectContext,
        dto::project_context_filter_dto::ProjectContextFilterDto,
        repositories::project_context_repository::ProjectContextRepository,
    },
};

#[derive(Clone, Debug)]
pub struct ProjectContextService {
    project_context_repository: ProjectContextRepository,
}

impl ProjectContextService {
    pub fn new(project_context_repository: ProjectContextRepository) -> Self {
        Self {
            project_context_repository,
        }
    }

    // TODO: ajouter test sur la pagination (mettre de la merde) + voir si fonctione sans slug after
    pub async fn get_ordered_project_contexts(
        &self,
        pagination: CursorPaginationDto,
        filter: ProjectContextFilterDto,
    ) -> Result<(Vec<ProjectContext>, u32), sqlx::Error> {
        let ordered_projects = self
            .project_context_repository
            .get_project_contexts(&pagination, &filter)
            .await?;

        let total_count = self
            .project_context_repository
            .get_project_context_total_count(&filter)
            .await?;

        Ok((ordered_projects, total_count))
    }

    /**
     * It returns the relevant projects' contexts
     * We consider the firsts as the relevant projects
     */
    pub async fn get_relevant_project_contexts(
        &self,
        limit: u32,
    ) -> Result<Vec<ProjectContext>, sqlx::Error> {
        let pagination = CursorPaginationDto::new_from_limit(limit);
        let filter = ProjectContextFilterDto::default();

        let ordered_projects = self
            .project_context_repository
            .get_project_contexts(&pagination, &filter)
            .await?;

        Ok(ordered_projects)
    }
}
