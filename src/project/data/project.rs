use std::{
    error::Error,
    fs::{self, DirEntry},
};

use regex::Regex;
use serde::{Deserialize, Serialize};

use super::{project_content::ProjectContent, project_context::ProjectContext};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Project {
    pub context: ProjectContext,
    pub content: ProjectContent,
}

impl Project {
    fn split_markdown_document(
        document: &str,
    ) -> Result<(&str, &str), regex::Error> {
        let separator_regex = Regex::new(r"-{3}")?;

        let splited_values: Vec<_> = separator_regex.split(document).collect();

        let mut raw_project_context = "";
        let mut raw_project_content = "";

        for splited_value in splited_values {
            if splited_value.is_empty() {
                continue;
            }

            if raw_project_context.is_empty() {
                raw_project_context = splited_value.trim();
                continue;
            }

            if raw_project_content.is_empty() {
                raw_project_content = splited_value.trim();
                continue;
            }

            panic!("Invalid Markdown content, more than two parts")
        }

        Ok((raw_project_context, raw_project_content))
    }

    pub async fn parse_from_markdown_data(
        data: &str,
    ) -> Result<Self, Box<dyn Error>> {
        let (raw_project_context, raw_project_content) =
            Project::split_markdown_document(data)?;

        let project_context =
            ProjectContext::parse_from_yaml_data(raw_project_context).await?;

        let project_content =
            ProjectContent::parse_from_str_data(raw_project_content)?;

        Ok(Self {
            context: project_context,
            content: project_content,
        })
    }

    pub async fn parse_from_markdown_file(
        file: DirEntry,
    ) -> Result<Self, Box<dyn Error>> {
        let raw_project = fs::read_to_string(file.path())?;

        let project = Project::parse_from_markdown_data(&raw_project).await?;

        Ok(project)
    }
}
