use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::{
    error::BoxDynError,
    sqlite::{SqliteTypeInfo, SqliteValueRef},
    Decode, Sqlite, Type,
};

use super::project_tag::ProjectTag;

#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ProjectTags(pub Vec<ProjectTag>);

impl ProjectTags {
    fn from_string(string: String) -> ProjectTags {
        let tags = string.split(",").map(|tag| tag.into()).collect();

        ProjectTags(tags)
    }
}

impl From<Vec<String>> for ProjectTags {
    fn from(project_tags: Vec<String>) -> ProjectTags {
        let project_tags: Vec<ProjectTag> =
            project_tags.iter().map(|tag| tag.into()).collect();

        ProjectTags(project_tags)
    }
}

impl From<Vec<ProjectTag>> for ProjectTags {
    fn from(project_tags: Vec<ProjectTag>) -> ProjectTags {
        ProjectTags(project_tags)
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
impl Type<Sqlite> for ProjectTags {
    fn type_info() -> SqliteTypeInfo {
        <String as Type<Sqlite>>::type_info()
    }
}

#[cfg(feature = "ssr")]
impl<'row> Decode<'row, Sqlite> for ProjectTags {
    fn decode(value: SqliteValueRef<'row>) -> Result<Self, BoxDynError> {
        let value: String = <&str as Decode<Sqlite>>::decode(value)?.into();

        Ok(value.into())
    }
}
