use crate::{
    project::data::project_context::ProjectContext,
    system::local_database::LocalDatabase,
};

#[derive(Clone, Debug)]
pub struct ProjectContextRepository {
    local_database_uri: String,
}

impl ProjectContextRepository {
    pub fn new(local_database_uri: String) -> Self {
        Self { local_database_uri }
    }

    /**
     * It uses a cursor-based pagination
     */
    pub async fn get_project_contexts(
        &self,
        pagination_limit: u32,
        slug_cursor_after: Option<String>,
    ) -> Result<Vec<ProjectContext>, sqlx::Error> {
        let mut local_database =
            LocalDatabase::new(&self.local_database_uri).await?;

        let project_contexts = match slug_cursor_after {
            Some(slug_cursor_after) => {
                sqlx::query_as!(
                    ProjectContext,
                    "
                    SELECT    `projects`.`slug`,
                            `projects`.`next_slug`,
                            `projects`.`title`,
                            `projects`.`image_url`,
                            `projects`.`date`,
                            `project_tags`.`name` AS `tags`
                    FROM      `projects`
                    LEFT JOIN `project_tags` ON `project_tags`.`project_slug` \
                     = `projects`.`slug`
                    WHERE     `projects`.`position` >= (
                            SELECT    `projects`.`position`
                            FROM      `projects`
                            WHERE     `projects`.`slug` = ?
                            )
                    GROUP BY  `projects`.`slug`
                    ORDER BY  position ASC
                    LIMIT     ?;
                    ",
                    slug_cursor_after,
                    pagination_limit
                )
                .fetch_all(&mut local_database.conn)
                .await?
            }
            None => {
                sqlx::query_as!(
                    ProjectContext,
                    "
                    SELECT    `projects`.`slug`,
                            `projects`.`next_slug`,
                            `projects`.`title`,
                            `projects`.`image_url`,
                            `projects`.`date`,
                            `project_tags`.`name` AS `tags`
                    FROM      `projects`
                    LEFT JOIN `project_tags` ON `project_tags`.`project_slug` \
                     = `projects`.`slug`
                    GROUP BY  `projects`.`slug`
                    ORDER BY  position ASC
                    LIMIT     ?;
                    ",
                    pagination_limit
                )
                .fetch_all(&mut local_database.conn)
                .await?
            }
        };

        Ok(project_contexts)
    }
}

impl Default for ProjectContextRepository {
    fn default() -> Self {
        ProjectContextRepository::new("sqlite:local.db".into())
    }
}
