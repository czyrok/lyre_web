use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProjectTags(pub Vec<String>);

impl ProjectTags {
    fn from_string(string: String) -> ProjectTags {
        let tags = string.split(",").map(|tag| tag.into()).collect();

        ProjectTags(tags)
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
