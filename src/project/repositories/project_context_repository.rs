use chrono::Datelike;
use sqlx::{QueryBuilder, Row, Sqlite};

use crate::{
    core::dto::cursor_pagination_dto::CursorPaginationDto,
    project::{
        data::project_context::ProjectContext,
        dto::project_context_filter_dto::ProjectContextFilterDto,
        enums::implementation_year::ImplementationYear,
    },
    system::{
        database::local_database::LocalDatabase,
        state::environment_context::EnvironmentContext,
    },
};

#[derive(Clone, Debug)]
pub struct ProjectContextRepository {
    environment: EnvironmentContext,
}

impl ProjectContextRepository {
    pub fn new(environment: EnvironmentContext) -> Self {
        Self { environment }
    }

    // TODO: make test to check the sql string...
    // y'a des exemples ici https://docs.rs/sqlx/latest/sqlx/struct.QueryBuilder.html
    pub fn apply_filters_on_query(
        query_builder: &'_ mut QueryBuilder<'_, Sqlite>,
        filter: &ProjectContextFilterDto,
    ) {
        if let Some(searched_project_title) =
            filter.searched_project_title.clone()
        {
            query_builder.push(" AND (`projects`.`title` LIKE ");
            query_builder.push_bind(format!("%{}%", searched_project_title));
            query_builder.push(" ) ");
        }

        let has_filter_tags = !filter.tags.is_empty();

        if has_filter_tags {
            let mut separated_query_builder = query_builder.separated(", ");

            separated_query_builder.push_unseparated(
                " AND ( ( SELECT COUNT(*) FROM `project_tags` WHERE \
                 `project_tags`.`project_slug` = `projects`.`slug` AND \
                 `project_tags`.`name` IN ( ",
            );

            for tag in filter.tags.iter() {
                separated_query_builder.push_bind(tag.clone());
            }

            separated_query_builder.push_unseparated(") ) > 0 ) ");
        }

        let has_filter_implementation_years =
            !filter.implementation_years.is_empty();

        if has_filter_implementation_years {
            let current_date = chrono::Utc::now();
            let current_year = current_date.year();

            let mut separated_query_builder = query_builder.separated(" OR ");

            separated_query_builder.push_unseparated(" AND (");

            for implementation_year in filter.implementation_years.iter() {
                let year_query = match implementation_year {
                    ImplementationYear::CurrentYear => {
                        format!(" `projects`.`date` LIKE '{}-%' ", current_year)
                    }
                    ImplementationYear::LastYear => format!(
                        " `projects`.`date` LIKE '{}-%' ",
                        current_year - 1
                    ),
                    ImplementationYear::TwoYearsAgo => format!(
                        " `projects`.`date` LIKE '{}-%' ",
                        current_year - 2
                    ),
                    ImplementationYear::MoreThanTwoYears => format!(
                        " (`projects`.`date` NOT LIKE '{}-%' AND \
                         `projects`.`date` NOT LIKE '{}-%' AND \
                         `projects`.`date` NOT LIKE '{}-%' )",
                        current_year,
                        current_year - 1,
                        current_year - 2
                    ),
                };

                separated_query_builder.push(year_query);
            }

            separated_query_builder.push_unseparated(") ");
        }
    }

    pub fn apply_pagination_cursor_after_on_query(
        query_builder: &'_ mut QueryBuilder<'_, Sqlite>,
        pagination: &CursorPaginationDto,
    ) {
        if let Some(cursor_after) = pagination.cursor_after.clone() {
            query_builder.push(
                "
                AND `projects`.`position` >= (
                    SELECT `projects`.`position`
                    FROM `projects`
                    WHERE `projects`.`slug` =
                ",
            );
            query_builder.push_bind(cursor_after);

            query_builder.push(") ");
        }
    }

    pub fn apply_pagination_limit(
        query_builder: &'_ mut QueryBuilder<'_, Sqlite>,
        pagination: &CursorPaginationDto,
    ) {
        query_builder.push("LIMIT ");
        query_builder.push_bind(pagination.limit);
    }

    /**
     * It uses a cursor-based pagination
     */
    pub async fn get_project_contexts(
        &self,
        pagination: &CursorPaginationDto,
        filter: &ProjectContextFilterDto,
    ) -> Result<Vec<ProjectContext>, sqlx::Error> {
        let mut local_database =
            LocalDatabase::new(&self.environment.local_database_uri).await?;

        let mut query_builder: QueryBuilder<'_, Sqlite> = QueryBuilder::new(
            "
            SELECT    `projects`.`slug`,
                    `projects`.`next_slug`,
                    (
                    SELECT    `projects_for_next`.`title`
                    FROM      `projects` AS `projects_for_next`
                    WHERE     `projects_for_next`.`slug` = \
             `projects`.`next_slug`
                    ) AS `next_title`,
                    `projects`.`title`,
                    `projects`.`image_url`,
                    `projects`.`date`,
                    ---- https://www.sqlitetutorial.net/sqlite-json-functions/sqlite-json_group_array-function/
                    JSON_GROUP_ARRAY (`project_tags`.`name`) AS `tags`
            FROM      `projects`
            INNER JOIN `project_tags` ON `project_tags`.`project_slug` = \
             `projects`.`slug`
             ",
        );

        query_builder.push(" WHERE true = true ");
        ProjectContextRepository::apply_filters_on_query(
            &mut query_builder,
            filter,
        );
        ProjectContextRepository::apply_pagination_cursor_after_on_query(
            &mut query_builder,
            pagination,
        );

        query_builder.push(
            "
            GROUP BY  `projects`.`slug`
            ORDER BY  `position` ASC
            ",
        );
        ProjectContextRepository::apply_pagination_limit(
            &mut query_builder,
            pagination,
        );

        let query = query_builder.build_query_as();

        let project_contexts =
            query.fetch_all(&mut local_database.connection).await?;

        Ok(project_contexts)
    }

    pub async fn get_project_context_total_count(
        &self,
        filter: &ProjectContextFilterDto,
    ) -> Result<u32, sqlx::Error> {
        let mut local_database =
            LocalDatabase::new(&self.environment.local_database_uri).await?;

        let mut query_builder: QueryBuilder<'_, Sqlite> = QueryBuilder::new(
            "
            SELECT    COUNT(DISTINCT `projects`.`slug`) AS count
            FROM      `projects`
            LEFT JOIN `project_tags` ON `project_tags`.`project_slug` = \
             `projects`.`slug`
            ",
        );

        query_builder.push(" WHERE true = true ");
        ProjectContextRepository::apply_filters_on_query(
            &mut query_builder,
            filter,
        );

        let query = query_builder.build();

        let row = query.fetch_optional(&mut local_database.connection).await?;

        let count: u32 = match row {
            Some(row) => row.try_get("count")?,
            None => 0,
        };

        Ok(count)
    }
}
