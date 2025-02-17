use crate::system::{
    database::local_database::LocalDatabase,
    state::environment_context::EnvironmentContext,
};

#[derive(Clone, Debug)]
pub struct ProjectSlugRepository {
    environment: EnvironmentContext,
}

impl ProjectSlugRepository {
    pub fn new(environment: EnvironmentContext) -> Self {
        Self { environment }
    }

    pub async fn get_project_slugs(&self) -> Result<Vec<String>, sqlx::Error> {
        let mut local_database =
            LocalDatabase::new(&self.environment.local_database_uri).await?;

        let project_slugs = sqlx::query!(
            "
                SELECT slug FROM `projects`;
                ",
        )
        .map(|row| row.slug)
        .fetch_all(&mut local_database.connection)
        .await?;

        Ok(project_slugs)
    }
}
