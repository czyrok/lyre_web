use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::{
    error::BoxDynError,
    sqlite::{SqliteTypeInfo, SqliteValueRef},
    Decode, Sqlite, Type,
};

use super::project_link::ProjectLink;

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct ProjectLinks(pub Vec<ProjectLink>);

impl ProjectLinks {
    #[cfg(feature = "ssr")]
    pub fn parse_from_yaml_data(data: &str) -> Result<Self, serde_yml::Error> {
        use serde_yml::Value;

        let deserialized_metadata: Value = serde_yml::from_str(data)?;

        let links_field_value = deserialized_metadata.get("links");

        if let Some(links_field_value) = links_field_value {
            let deserialized_links: ProjectLinks =
                serde_yml::from_value(links_field_value.clone())?;

            return Ok(deserialized_links);
        }

        Ok(ProjectLinks::default())
    }
}

#[cfg(feature = "ssr")]
impl Type<Sqlite> for ProjectLinks {
    fn type_info() -> SqliteTypeInfo {
        <String as Type<Sqlite>>::type_info()
    }
}

#[cfg(feature = "ssr")]
impl<'row> Decode<'row, Sqlite> for ProjectLinks {
    fn decode(value_ref: SqliteValueRef<'row>) -> Result<Self, BoxDynError> {
        let value_string: String =
            <&str as Decode<Sqlite>>::decode(value_ref)?.into();

        let value = serde_json::from_str(&value_string).unwrap();

        Ok(value)
    }
}
