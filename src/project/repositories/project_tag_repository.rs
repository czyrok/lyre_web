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

    pub async fn save_project_tag(
        &self,
        project_slug: String,
        tag: ProjectTag,
        local_database_transaction: &mut LocalDatabaseTransaction<'_>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "
                INSERT INTO `project_tags` (`project_slug`, `name`) VALUES (?, \
             ?);
                ",
            project_slug,
            tag.name
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
            SELECT    DISTINCT `name`
            FROM      `project_tags`;
            ",
        )
        .fetch_all(&mut local_database.connection)
        .await?;

        Ok(project_tags.into())
    }
}
