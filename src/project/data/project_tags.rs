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

impl From<Vec<ProjectTag>> for ProjectTags {
    fn from(project_tags: Vec<ProjectTag>) -> ProjectTags {
        ProjectTags(project_tags)
    }
}

impl From<Vec<String>> for ProjectTags {
    fn from(project_tags: Vec<String>) -> ProjectTags {
        let project_tags: Vec<ProjectTag> =
            project_tags.iter().map(|tag| tag.into()).collect();

        ProjectTags(project_tags)
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
    fn decode(value_ref: SqliteValueRef<'row>) -> Result<Self, BoxDynError> {
        let value_string: String =
            <&str as Decode<Sqlite>>::decode(value_ref)?.into();

        let tag_values: Vec<String> =
            serde_json::from_str(&value_string).unwrap();

        Ok(tag_values.into())
    }
}
