use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProjectContext {
    pub title: String,
    pub slug: String,
    #[serde(skip_serializing)]
    pub order: Option<u8>,
    pub image_url: String,
    pub date: String,
    pub tags: Vec<String>,
}

impl ProjectContext {
    pub async fn parse_from_yaml_data(
        data: &str,
    ) -> Result<Self, serde_yml::Error> {
        let deserialized_context: Self = serde_yml::from_str(data)?;

        Ok(deserialized_context)
    }
}
