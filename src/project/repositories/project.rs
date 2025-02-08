use std::{
    error::Error,
    fs::{self, DirEntry},
    io,
};

use crate::{
    project::data::{
        project::Project, project_content::ProjectContent,
        project_context::ProjectContext,
    },
    system::local_database::LocalDatabase,
};

#[derive(Clone, Debug)]
pub struct ProjectRepository {
    local_database_uri: String,
    project_data_path: String,
}

// TODO: test à faire => slug unique
// TODO: test à faire => order unique + dans l'ordre de 0..X sans nombre en moins

impl ProjectRepository {
    pub fn new(local_database_uri: String, project_data_path: String) -> Self {
        Self {
            local_database_uri,
            project_data_path,
        }
    }

    fn get_project_data_entries(&self) -> Result<Vec<DirEntry>, io::Error> {
        let project_data_dir = fs::read_dir(self.project_data_path.clone())?;

        let mut project_data_files = vec![];

        for project_data_file in project_data_dir {
            project_data_files.push(project_data_file?);
        }

        Ok(project_data_files)
    }

    pub async fn read_project_data(
        &self,
    ) -> Result<Vec<Project>, Box<dyn Error>> {
        let mut projects = vec![];

        let project_data_files = self.get_project_data_entries()?;

        for project_data_file in project_data_files {
            let project =
                Project::parse_from_markdown_file(project_data_file).await?;

            projects.push(project);
        }

        Ok(projects)
    }

    pub async fn clean_projects(&self) -> Result<(), sqlx::Error> {
        let mut local_database =
            LocalDatabase::new(&self.local_database_uri).await?;

        sqlx::query!(
            "
                DELETE FROM `projects`;
                "
        )
        .fetch_optional(&mut local_database.conn)
        .await?;

        Ok(())
    }

    pub async fn save_project(
        &self,
        project: Project,
    ) -> Result<(), sqlx::Error> {
        let mut local_database =
            LocalDatabase::new(&self.local_database_uri).await?;

        sqlx::query!(
            "
                INSERT INTO `projects` (`slug`, `next_slug`, `position`, \
             `title`, `image_url`, `date`, `content`) VALUES (?, ?, ?, ?, ?, \
             ?, ?);
                ",
            project.context.slug,
            project.context.next_slug,
            project.position,
            project.context.title,
            project.context.image_url,
            project.context.date,
            project.content.0
        )
        .fetch_optional(&mut local_database.conn)
        .await?;

        Ok(())
    }

    pub async fn exists_project_from_slug(
        &self,
        slug: &str,
    ) -> Result<bool, sqlx::Error> {
        let mut local_database =
            LocalDatabase::new(&self.local_database_uri).await?;

        let result = sqlx::query!(
            "
                SELECT    `projects`.`slug`
                FROM      `projects`
                WHERE     `projects`.`slug` = ?;
                ",
            slug
        )
        .fetch_optional(&mut local_database.conn)
        .await?;

        match result {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    pub async fn get_project(
        &self,
        slug: &str,
    ) -> Result<Option<Project>, sqlx::Error> {
        let mut local_database =
            LocalDatabase::new(&self.local_database_uri).await?;

        let project = sqlx::query!(
            "
                SELECT    `projects`.`slug`,
                        `projects`.`next_slug`,
                        `projects`.`title`,
                        `projects`.`image_url`,
                        `projects`.`date`,
                        `projects`.`content`,
                        `project_tags`.`name` AS `tags`
                FROM      `projects`
                LEFT JOIN `project_tags` ON `project_tags`.`project_slug` = \
             `projects`.`slug`
                WHERE     `projects`.`slug` = ?;
                ",
            slug
        )
        .map(|row| Project {
            position: None,
            context: ProjectContext {
                slug: row.slug,
                next_slug: row.next_slug,
                title: row.title,
                image_url: row.image_url,
                date: row.date,
                tags: row.tags.into(),
            },
            content: ProjectContent(row.content),
        })
        .fetch_optional(&mut local_database.conn)
        .await?;

        Ok(project)
    }
}

impl Default for ProjectRepository {
    fn default() -> Self {
        ProjectRepository::new("sqlite:local.db".into(), "project_data/".into())
    }
}
