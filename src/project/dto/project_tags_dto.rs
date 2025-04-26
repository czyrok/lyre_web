use serde::{Deserialize, Serialize};

use crate::project::data::project_tags::ProjectTags;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct ProjectTagsDto {
    pub project_tags: ProjectTags,
}

impl ProjectTagsDto {
    pub fn new(project_tags: ProjectTags) -> Self {
        Self { project_tags }
    }
}
