use serde::{Deserialize, Serialize};

use crate::project::enums::implementation_year::ImplementationYear;

#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ProjectContextFilterDto {
    pub searched_project_title: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub implementation_years: Vec<ImplementationYear>,
}

impl ProjectContextFilterDto {
    pub fn new(
        searched_project_title: Option<String>,
        tags: Vec<String>,
        implementation_years: Vec<ImplementationYear>,
    ) -> Self {
        Self {
            searched_project_title,
            tags,
            implementation_years,
        }
    }
}
