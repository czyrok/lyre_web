use std::{
    error::Error,
    fs::{self, DirEntry},
    io,
};

use leptos::logging::error;

use crate::{
    project::data::project::Project,
    system::{
        database::{
            local_database::LocalDatabase,
            local_database_transaction::LocalDatabaseTransaction,
        },
        state::environment_context::EnvironmentContext,
    },
};

#[derive(Clone, Debug)]
pub struct ProjectRepository {
    environment: EnvironmentContext,
}

// TODO: test à faire => slug unique
// TODO: test à faire => order unique + dans l'ordre de 0..X sans nombre en moins

impl ProjectRepository {
    pub fn new(environment: EnvironmentContext) -> Self {
        Self { environment }
    }

    fn get_project_data_entries(&self) -> Result<Vec<DirEntry>, io::Error> {
        let project_data_dir =
            fs::read_dir(self.environment.project_data_dir_path.clone())?;

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
            let file_name = project_data_file.file_name();

            let parse_result =
                Project::parse_from_markdown_file(project_data_file).await;

            match parse_result {
                Ok(project) => {
                    projects.push(project);
                }
                Err(error) => {
                    error!(
                        "Unable to parse this project `{:?}`, \n{:?}",
                        file_name, error
                    );

                    return Err(error);
                }
            }
        }

        Ok(projects)
    }

    pub async fn clean_projects(
        &self,
        local_database_transaction: &mut LocalDatabaseTransaction<'_>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "
                DELETE FROM `projects`;
                "
        )
        .fetch_optional(&mut *local_database_transaction.value)
        .await?;

        Ok(())
    }

    pub async fn save_project(
        &self,
        project: Project,
        local_database_transaction: &mut LocalDatabaseTransaction<'_>,
    ) -> Result<(), sqlx::Error> {
        let next_slug = project.context.next.map(|next| next.slug);

        sqlx::query!(
            "
                INSERT INTO `projects` (`slug`, `next_slug`, `position`, \
             `title`, `image_url`, `date`, `content`) VALUES (?, ?, ?, ?, ?, \
             ?, ?);
                ",
            project.context.slug,
            next_slug,
            project.position,
            project.context.title,
            project.context.image_url,
            project.context.date,
            project.content.0
        )
        .fetch_optional(&mut *local_database_transaction.value)
        .await?;

        Ok(())
    }

    pub async fn exists_project_from_slug(
        &self,
        slug: &str,
    ) -> Result<bool, sqlx::Error> {
        let mut local_database =
            LocalDatabase::new(&self.environment.local_database_uri).await?;

        let result = sqlx::query!(
            "
                SELECT    `projects`.`slug`
                FROM      `projects`
                WHERE     `projects`.`slug` = ?;
                ",
            slug
        )
        .fetch_optional(&mut local_database.connection)
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
            LocalDatabase::new(&self.environment.local_database_uri).await?;

        let project = sqlx::query_as::<_, Project>(
            "
            SELECT    `projects`.`slug`,
                    `projects`.`next_slug`,
                    (
                    SELECT    `projects_for_next`.`title`
                    FROM      `projects` AS `projects_for_next`
                    WHERE     `projects_for_next`.`slug` = `projects`.`next_slug`
                    ) AS `next_title`,
                    `projects`.`title`,
                    `projects`.`image_url`,
                    `projects`.`date`,
                    `projects`.`content`,
                    (
                    ---- https://www.sqlitetutorial.net/sqlite-json-functions/sqlite-json_group_array-function/
                    SELECT    JSON_GROUP_ARRAY (`project_tags`.`name`)
                    FROM      `project_tags`
                    WHERE     `project_tags`.`project_slug` = `projects`.`slug`
                    ) AS `tags`,
                    (
                    ---- https://www.sqlitetutorial.net/sqlite-json-functions/sqlite-json_group_array-function/
                    ---- https://www.sqlitetutorial.net/sqlite-json-functions/sqlite-json_object-function/
                    SELECT    JSON_GROUP_ARRAY (
                                JSON_OBJECT('url', `project_links`.`url`, 'title', `project_links`.`title`, 'icon', `project_links`.`icon`)
                                )
                    FROM      `project_links`
                    WHERE     `project_links`.`project_slug` = `projects`.`slug`
                    ) AS `links`
            FROM      `projects`
            WHERE     `projects`.`slug` = ?;
            ",
        )
        .bind(slug)
        .fetch_optional(&mut local_database.connection)
        .await?;

        Ok(project)
    }
}
