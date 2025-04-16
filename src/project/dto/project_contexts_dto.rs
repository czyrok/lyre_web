use serde::{Deserialize, Serialize};

use crate::project::data::project_context::ProjectContext;

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct ProjectContextsDto {
    pub project_contexts: Vec<ProjectContext>,
    pub total_count: u32,
}

impl ProjectContextsDto {
    pub fn new(
        project_contexts: Vec<ProjectContext>,
        total_count: u32,
    ) -> Self {
        Self {
            project_contexts,
            total_count,
        }
    }
}
