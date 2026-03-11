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
        use crate::core::data::html_formatter::HtmlFormatter;

        let arena = Arena::new();

        let deserialized_markdown =
            comrak::parse_document(&arena, data, &Options::default());

        let mut content = String::new();

        HtmlFormatter::format_document(
            deserialized_markdown,
            &Options::default(),
            &mut content,
        )
        .unwrap();

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
