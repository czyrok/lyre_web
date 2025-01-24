use serde::{Deserialize, Serialize};

use crate::project::data::project_context::ProjectContext;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct ProjectContextsDto {
    pub project_contexts: Vec<ProjectContext>,
}

impl ProjectContextsDto {
    pub fn new(project_contexts: Vec<ProjectContext>) -> Self {
        return Self { project_contexts };
    }
}
