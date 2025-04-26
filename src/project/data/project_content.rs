#[cfg(feature = "ssr")]
use std::string::FromUtf8Error;

#[cfg(feature = "ssr")]
use comrak::{Arena, Options};
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::{sqlite::SqliteRow, FromRow, Row};

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct ProjectContent(pub String);

impl ProjectContent {
    #[cfg(feature = "ssr")]
    pub fn parse_from_markdown_data(data: &str) -> Result<Self, FromUtf8Error> {
        let arena = Arena::new();

        let deserialized_markdown =
            comrak::parse_document(&arena, data, &Options::default());

        let mut raw_content: Vec<u8> = vec![];

        comrak::format_html(
            deserialized_markdown,
            &Options::default(),
            &mut raw_content,
        )
        .unwrap();

        let content = String::from_utf8(raw_content)?;

        Ok(Self(content))
    }
}

#[cfg(feature = "ssr")]
impl<'row> FromRow<'row, SqliteRow> for ProjectContent {
    fn from_row(row: &'row SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(ProjectContent(
            row.try_get("content").expect("`row.content` should exist"),
        ))
    }
}
