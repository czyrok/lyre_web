#[cfg(feature = "ssr")]
use std::error::Error;

#[cfg(feature = "ssr")]
use leptos::logging::log;

#[cfg(feature = "ssr")]
use crate::project::{
    data::project::Project,
    repositories::{
        project::ProjectRepository, project_tag::ProjectTagRepository,
    },
};

#[derive(Clone, Debug)]
pub struct ProjectService {
    #[cfg(feature = "ssr")]
    project_repository: ProjectRepository,
    #[cfg(feature = "ssr")]
    project_tag_repository: ProjectTagRepository,
}

impl ProjectService {
    #[cfg(feature = "ssr")]
    pub fn new(
        project_repository: ProjectRepository,
        project_tag_repository: ProjectTagRepository,
    ) -> Self {
        Self {
            project_repository,
            project_tag_repository,
        }
    }

    #[cfg(feature = "ssr")]
    pub async fn refresh_project_cache(
        &mut self,
    ) -> Result<(), Box<dyn Error>> {
        self.project_tag_repository.clean_project_tags().await?;
        self.project_repository.clean_projects().await?;

        self.cache_project_data().await?;

        Ok(())
    }

    #[cfg(feature = "ssr")]
    async fn cache_project(&self, project: Project) -> Result<(), sqlx::Error> {
        self.project_repository
            .save_project(project.clone())
            .await?;

        log!("Project `{}` cached", project.context.slug);

        for tag in project.context.tags.0 {
            self.project_tag_repository
                .save_project_tag(project.context.slug.clone(), tag)
                .await?;
        }

        Ok(())
    }

    #[cfg(feature = "ssr")]
    async fn cache_project_data(&mut self) -> Result<(), Box<dyn Error>> {
        let projects = self.project_repository.read_project_data().await?;

        let mut sorted_projects = Project::sort_projects(projects);

        //// We need to reverse the array to avoid to have the next project available
        sorted_projects.reverse();

        // TODO: transaction
        for project in sorted_projects {
            self.cache_project(project).await?;
        }

        Ok(())
    }

    #[cfg(feature = "ssr")]
    pub async fn exists_project_from_slug(
        &self,
        slug: &str,
    ) -> Result<bool, sqlx::Error> {
        let exists = self
            .project_repository
            .exists_project_from_slug(slug)
            .await?;

        Ok(exists)
    }

    #[cfg(feature = "ssr")]
    pub async fn get_project(
        &self,
        slug: &str,
    ) -> Result<Option<Project>, sqlx::Error> {
        let project = self.project_repository.get_project(slug).await?;

        Ok(project)
    }
}
