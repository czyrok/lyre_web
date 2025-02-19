use crate::system::database::local_database_transaction::LocalDatabaseTransaction;

#[derive(Default, Clone, Debug)]
pub struct ProjectTagRepository {}

impl ProjectTagRepository {
    pub async fn save_project_tag(
        &self,
        project_slug: String,
        tag: String,
        local_database_transaction: &mut LocalDatabaseTransaction<'_>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "
                INSERT INTO `project_tags` (`project_slug`, `name`) VALUES (?, \
             ?);
                ",
            project_slug,
            tag
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
}
