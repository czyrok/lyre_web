use super::{
    project::Project, project_context::ProjectContext,
    project_repository::ProjectRepository,
};

#[derive(Clone, Debug)]
pub struct ProjectService {
    project_repository: ProjectRepository,
}

impl ProjectService {
    pub fn new(project_repository: ProjectRepository) -> Self {
        Self { project_repository }
    }

    pub fn get_all_ordered_project_contexts(&self) -> Vec<ProjectContext> {
        let cached_projects = self.project_repository.get_cached_projects();

        cached_projects
            .iter()
            .map(|project| project.context.clone())
            .collect()
    }

    pub fn get_project_slugs(&self) -> Vec<String> {
        let cached_projects = self.project_repository.get_cached_projects();

        cached_projects
            .iter()
            .map(|project| project.context.slug.clone())
            .collect()
    }

    /**
     * It uses a cursor-based pagination
     * TODO: ajouter test sur la pagination (mettre de la merde) + voir si fonctione sans slug after
     */
    pub fn get_ordered_project_contexts(
        &self,
        mut pagination_limit: usize,
        slug_cursor_after: Option<String>,
    ) -> Vec<ProjectContext> {
        let ordered_project_contexts = self.get_all_ordered_project_contexts();

        if pagination_limit > ordered_project_contexts.len() {
            pagination_limit = ordered_project_contexts.len()
        }

        let pagination_offset = match slug_cursor_after {
            Some(slug_cursor_after) => ordered_project_contexts
                .iter()
                .position(|project| project.slug == slug_cursor_after)
                .unwrap(),
            None => 0,
        };

        let paginated_project_contexts =
            &ordered_project_contexts[pagination_offset..pagination_limit];

        paginated_project_contexts.to_vec()
    }

    pub fn exists_project_from_slug(&self, slug: &str) -> bool {
        let all_project_contexts = self.get_all_ordered_project_contexts();

        all_project_contexts
            .iter()
            .any(|project| project.slug == slug)
    }

    pub fn get_project(&self, slug: &str) -> Option<Project> {
        let all_projects = self.project_repository.get_cached_projects();

        if let Some(project) = all_projects
            .iter()
            .find(|project| project.context.slug == slug)
        {
            return Some(project.to_owned());
        }

        None
    }

    /**
     * It returns the relevant projects' contexts
     * We consider the firsts as the relevant projects
     */
    pub fn get_relevant_project_contexts(
        &self,
        limit: usize,
    ) -> Vec<ProjectContext> {
        self.get_ordered_project_contexts(limit, None)
    }
}

impl Default for ProjectService {
    fn default() -> Self {
        ProjectService::new(ProjectRepository::default())
    }
}
