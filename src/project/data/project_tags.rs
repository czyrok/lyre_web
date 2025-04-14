use std::error::Error;

use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::{Database, Decode};

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct ProjectTags(pub Vec<String>);

impl ProjectTags {
    fn from_string(string: String) -> ProjectTags {
        let tags = string.split(",").map(|tag| tag.into()).collect();

        ProjectTags(tags)
    }
}

impl From<Option<ProjectTags>> for ProjectTags {
    fn from(project_tags: Option<ProjectTags>) -> ProjectTags {
        project_tags.unwrap_or_default()
    }
}

impl From<Option<String>> for ProjectTags {
    fn from(string: Option<String>) -> ProjectTags {
        if let Some(string) = string {
            return ProjectTags::from_string(string);
        }

        ProjectTags(vec![])
    }
}

impl From<String> for ProjectTags {
    fn from(string: String) -> ProjectTags {
        ProjectTags::from_string(string)
    }
}

#[cfg(feature = "ssr")]
impl<'row, DB: Database> Decode<'row, DB> for ProjectTags
where
    &'row str: Decode<'row, DB>,
{
    fn decode(
        value: <DB as Database>::ValueRef<'row>,
    ) -> Result<ProjectTags, Box<dyn Error + 'static + Send + Sync>> {
        let value: String = <&str as Decode<DB>>::decode(value)?.into();

        Ok(value.into())
    }
}
