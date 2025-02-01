use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct ProjectSlugsDto {
    pub project_slugs: Vec<String>,
}

impl ProjectSlugsDto {
    pub fn new(project_slugs: Vec<String>) -> Self {
        Self { project_slugs }
    }
}
