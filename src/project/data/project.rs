#[cfg(feature = "ssr")]
use std::{
    error::Error,
    fs::{self, DirEntry},
};

#[cfg(feature = "ssr")]
use regex::Regex;
use serde::{Deserialize, Serialize};

use super::{project_content::ProjectContent, project_context::ProjectContext};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Project {
    pub position: Option<u32>,
    pub context: ProjectContext,
    pub content: ProjectContent,
}

impl Project {
    #[cfg(feature = "ssr")]
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

    #[cfg(feature = "ssr")]
    pub async fn parse_from_markdown_data(
        slug: String,
        data: &str,
    ) -> Result<Self, Box<dyn Error>> {
        let (raw_project_context, raw_project_content) =
            Project::split_markdown_document(data)?;

        let project_context =
            ProjectContext::parse_from_yaml_data(slug, raw_project_context)
                .await?;

        let project_content =
            ProjectContent::parse_from_markdown_data(raw_project_content)?;

        Ok(Self {
            position: None,
            context: project_context,
            content: project_content,
        })
    }

    #[cfg(feature = "ssr")]
    pub async fn parse_from_markdown_file(
        file: DirEntry,
    ) -> Result<Self, Box<dyn Error>> {
        let raw_project = fs::read_to_string(file.path())?;

        let project_slug = file
            .file_name()
            .into_string()
            .unwrap_or_default()
            .replace(".md", "");

        let project =
            Project::parse_from_markdown_data(project_slug, &raw_project)
                .await?;

        Ok(project)
    }

    fn get_first_project(projects: Vec<Project>) -> Project {
        projects
            .clone()
            .into_iter()
            .find(|current_project| {
                let has_previous_project =
                    projects.clone().iter().any(|project| {
                        project.context.next_slug.clone().unwrap_or_default()
                            == current_project.context.slug
                    });

                !has_previous_project
            })
            .expect("A previous project should be existed")
    }

    fn get_next_project(
        previous_project: Project,
        projects: Vec<Project>,
    ) -> Project {
        projects
            .into_iter()
            .find(|project| {
                project.context.slug
                    == previous_project
                        .context
                        .next_slug
                        .clone()
                        .unwrap_or_default()
            })
            .expect("A next project should be existed")
    }

    // TODO: ajouter test sur l'ordre des projets
    pub fn sort_projects(projects: Vec<Project>) -> Vec<Project> {
        let projects_count = projects.len();
        let mut sorted_projects = vec![];

        let mut current_project_position = 1;

        let mut first_project = Self::get_first_project(projects.clone());

        first_project.position = Some(1);
        sorted_projects.push(first_project.clone());

        let mut previous_project = first_project;

        while sorted_projects.len() != projects_count {
            current_project_position += 1;

            let mut project =
                Self::get_next_project(previous_project, projects.clone());

            project.position = Some(current_project_position);
            sorted_projects.push(project.clone());

            previous_project = project;
        }

        sorted_projects
    }
}
