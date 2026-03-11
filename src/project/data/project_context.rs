use std::str::FromStr;

use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::{sqlite::SqliteRow, FromRow, Row};

use super::{next_project::NextProject, project_tags::ProjectTags};

#[derive(Default, Deserialize, Serialize, Clone, Debug)]
pub struct ProjectContext {
    pub slug: Option<String>,
    pub next: Option<NextProject>,
    pub title: String,
    pub image_url: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub formatted_date: Option<String>,
    pub tags: ProjectTags,
    pub meta_keywords: String,
    pub description: String,
}

impl ProjectContext {
    #[cfg(feature = "ssr")]
    pub async fn parse_from_yaml_data(
        slug: String,
        data: &str,
    ) -> Result<Self, serde_yml::Error> {
        let mut deserialized_context: Self = serde_yml::from_str(data)?;

        deserialized_context.slug = Some(slug);

        Ok(deserialized_context)
    }

    #[cfg(feature = "ssr")]
    pub fn complete_formatted_date(&mut self) {
        self.formatted_date = self.get_formatted_date().into();
    }

    //// `SSR` because it adds 4,8 Mo to the WASM in release mode
    #[cfg(feature = "ssr")]
    pub fn get_formatted_date(&self) -> String {
        use icu::{
            calendar::DateTime,
            datetime::{options::components, DateTimeFormatter},
            locid::Locale,
        };

        use crate::core::helpers::string::capitalize;

        let date = DateTime::try_new_iso_datetime(
            self.start_date.year(),
            self.start_date.month() as u8,
            self.start_date.day() as u8,
            0,
            0,
            0,
        )
        .expect("Failed to construct DateTime")
        .to_any();

        let mut bag = components::Bag::default();
        bag.month = Some(components::Month::Long);
        bag.year = Some(components::Year::Numeric);

        let options = icu::datetime::DateTimeFormatterOptions::Components(bag);

        let locale = Locale::from_str("fr-FR").unwrap();
        let formatter =
            DateTimeFormatter::try_new_experimental(&locale.into(), options)
                .expect("Failed to create DateTimeFormatter instance");

        let formated_date = formatter
            .format_to_string(&date)
            .expect("Calendars should match");

        capitalize(&formated_date)
    }
}

#[cfg(feature = "ssr")]
impl<'row> FromRow<'row, SqliteRow> for ProjectContext {
    fn from_row(row: &'row SqliteRow) -> Result<Self, sqlx::Error> {
        let next_slug = row.try_get::<Option<String>, &str>("next_slug")?;

        let next_project = match next_slug {
            Some(_) => Some(NextProject::from_row(row)?),
            None => None,
        };

        Ok(ProjectContext {
            slug: Some(row.try_get("slug")?),
            next: next_project,
            title: row.try_get("title").expect("`row.title` should exist"),
            image_url: row
                .try_get("image_url")
                .expect("`row.image_url` should exist"),
            start_date: row
                .try_get("start_date")
                .expect("`row.start_date` should exist"),
            end_date: row
                .try_get("end_date")
                .expect("`row.end_date` should exist"),
            formatted_date: None,
            tags: row.try_get("tags").expect("`row.tags` should exist"),
            meta_keywords: row.try_get("meta_keywords").unwrap_or_default(),
            description: row.try_get("description").unwrap_or_default(),
        })
    }
}
