use serde::{Deserialize, Serialize};

use crate::project::data::project::Project;

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct ProjectDto {
    pub project: Project,
}

impl ProjectDto {
    pub fn new(project: Project) -> Self {
        Self { project }
    }
}
