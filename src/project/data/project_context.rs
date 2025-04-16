use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::project_tags::ProjectTags;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct ProjectContext {
    //// We set the default value because the slug is not in the YAML metadata
    #[serde(default)]
    pub slug: String,
    pub next_slug: Option<String>,
    pub title: String,
    pub image_url: String,
    pub date: NaiveDate,
    pub tags: ProjectTags,
}

impl ProjectContext {
    #[cfg(feature = "ssr")]
    pub async fn parse_from_yaml_data(
        slug: String,
        data: &str,
    ) -> Result<Self, serde_yml::Error> {
        let mut deserialized_context: Self = serde_yml::from_str(data)?;

        deserialized_context.slug = slug;

        Ok(deserialized_context)
    }
}
