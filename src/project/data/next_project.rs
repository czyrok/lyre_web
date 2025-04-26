use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::{sqlite::SqliteRow, FromRow, Row};

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct NextProject {
    pub slug: String,
    pub title: Option<String>,
}

#[cfg(feature = "ssr")]
impl<'row> FromRow<'row, SqliteRow> for NextProject {
    fn from_row(row: &'row SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(NextProject {
            slug: row
                .try_get("next_slug")
                .expect("`row.next_slug` should exist"),
            title: row
                .try_get("next_title")
                .expect("`row.next_title` should exist"),
        })
    }
}
