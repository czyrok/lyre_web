use crate::system::local_database::LocalDatabase;

#[derive(Clone, Debug)]
pub struct ProjectSlugRepository {
    local_database_uri: String,
}

impl ProjectSlugRepository {
    pub fn new(local_database_uri: String) -> Self {
        Self { local_database_uri }
    }

    pub async fn get_project_slugs(&self) -> Result<Vec<String>, sqlx::Error> {
        let mut local_database =
            LocalDatabase::new(&self.local_database_uri).await?;

        let project_slugs = sqlx::query!(
            "
                SELECT slug FROM `projects`;
                ",
        )
        .map(|row| row.slug)
        .fetch_all(&mut local_database.conn)
        .await?;

        Ok(project_slugs)
    }
}

impl Default for ProjectSlugRepository {
    fn default() -> Self {
        ProjectSlugRepository::new("sqlite:local.db".into())
    }
}
