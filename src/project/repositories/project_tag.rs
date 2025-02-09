use crate::system::{
    environment_context::EnvironmentContext, local_database::LocalDatabase,
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
        tag: String,
    ) -> Result<(), sqlx::Error> {
        let mut local_database =
            LocalDatabase::new(&self.environment.local_database_uri).await?;

        sqlx::query!(
            "
                INSERT INTO `project_tags` (`project_slug`, `name`) VALUES (?, \
             ?);
                ",
            project_slug,
            tag
        )
        .fetch_optional(&mut local_database.conn)
        .await?;

        Ok(())
    }

    pub async fn clean_project_tags(&self) -> Result<(), sqlx::Error> {
        let mut local_database =
            LocalDatabase::new(&self.environment.local_database_uri).await?;

        sqlx::query!(
            "
                DELETE FROM `project_tags`;
                "
        )
        .fetch_optional(&mut local_database.conn)
        .await?;

        Ok(())
    }
}
