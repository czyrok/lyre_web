use crate::{
    project::data::project_link::ProjectLink,
    system::database::local_database_transaction::LocalDatabaseTransaction,
};

#[derive(Default, Clone, Debug)]
pub struct ProjectLinkRepository {}

impl ProjectLinkRepository {
    pub async fn save_project_link(
        &self,
        project_slug: String,
        link: ProjectLink,
        local_database_transaction: &mut LocalDatabaseTransaction<'_>,
    ) -> Result<(), sqlx::Error> {
        let icon_string: Option<String> =
            link.icon.clone().map(|icon| icon.into());

        sqlx::query(
            "
                INSERT INTO `project_links` (`project_slug`, `url`, `title`, \
             `icon`) VALUES (?, ?, ?, ?);
                ",
        )
        .bind(project_slug)
        .bind(link.url)
        .bind(link.title)
        .bind(icon_string)
        .fetch_optional(&mut *local_database_transaction.value)
        .await?;

        Ok(())
    }

    pub async fn clean_project_links(
        &self,
        local_database_transaction: &mut LocalDatabaseTransaction<'_>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "
            DELETE FROM `project_links`;
            "
        )
        .fetch_optional(&mut *local_database_transaction.value)
        .await?;

        Ok(())
    }
}
