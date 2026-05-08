use crate::{
    project::data::{project_tag::ProjectTag, project_tags::ProjectTags},
    system::{
        database::{
            local_database::LocalDatabase,
            local_database_transaction::LocalDatabaseTransaction,
        },
        state::environment_context::EnvironmentContext,
    },
};

#[derive(Clone, Debug)]
pub struct ProjectTagRepository {
    environment: EnvironmentContext,
}

impl ProjectTagRepository {
    pub fn new(environment: EnvironmentContext) -> Self {
        Self { environment }
    }

    pub async fn upsert_project_tag(
        &self,
        tag: ProjectTag,
        local_database_transaction: &mut LocalDatabaseTransaction<'_>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "
            INSERT INTO `project_tags` (`short_name`, `long_name`)
            VALUES(?, ?)
            ON CONFLICT (short_name) DO UPDATE SET
                long_name = excluded.long_name
            WHERE excluded.long_name IS NOT NULL;
            ",
            tag.short_name,
            tag.long_name
        )
        .fetch_optional(&mut *local_database_transaction.value)
        .await?;

        Ok(())
    }

    pub async fn clean_project_tags(
        &self,
        local_database_transaction: &mut LocalDatabaseTransaction<'_>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "
            DELETE FROM `project_tags`;
            "
        )
        .fetch_optional(&mut *local_database_transaction.value)
        .await?;

        Ok(())
    }

    pub async fn get_all_project_tags(
        &self,
    ) -> Result<ProjectTags, sqlx::Error> {
        let mut local_database =
            LocalDatabase::new(&self.environment.local_database_uri).await?;

        let project_tags = sqlx::query_as!(
            ProjectTag,
            "
            SELECT    `short_name`, `long_name`
            FROM      `project_tags`
            ORDER BY  `short_name` ASC, `long_name` ASC;
            ",
        )
        .fetch_all(&mut local_database.connection)
        .await?;

        Ok(project_tags.into())
    }
}
