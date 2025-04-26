use serde::{Deserialize, Serialize};

use crate::project::data::project_context::ProjectContext;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct RelevantProjectContextsDto {
    pub project_contexts: Vec<ProjectContext>,
}

impl RelevantProjectContextsDto {
    pub fn new(project_contexts: Vec<ProjectContext>) -> Self {
        Self { project_contexts }
    }
}
