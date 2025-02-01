use std::{
    error::Error,
    fs::{self, DirEntry},
    io,
};

use super::project::Project;

#[derive(Clone, Debug)]
pub struct ProjectRepository {
    project_data_path: String,
    cached_projects: Vec<Project>,
}

// TODO: test à faire => slug unique
// TODO: test à faire => order unique + dans l'ordre de 0..X sans nombre en moins

impl ProjectRepository {
    fn get_project_data_file_paths(&self) -> Result<Vec<DirEntry>, io::Error> {
        let project_data_dir = fs::read_dir(self.project_data_path.clone())?;

        let mut project_data_files = vec![];

        for project_data_file in project_data_dir {
            project_data_files.push(project_data_file?);
        }

        Ok(project_data_files)
    }

    // TODO: ajouter test sur l'ordre des projets
    fn sort_cached_projects(&mut self) {
        self.cached_projects
            .sort_by(|previous_project, current_project| {
                previous_project
                    .context
                    .order
                    .partial_cmp(&current_project.context.order)
                    .unwrap()
            });
    }

    pub fn new(project_data_path: &str) -> Self {
        Self {
            project_data_path: String::from(project_data_path),
            cached_projects: vec![],
        }
    }

    // TODO: useless ?
    pub async fn refresh_cache(&mut self) -> Result<(), Box<dyn Error>> {
        self.cached_projects.clear();

        self.cache_project_data().await?;

        Ok(())
    }

    pub async fn cache_project_data(&mut self) -> Result<(), Box<dyn Error>> {
        let project_data_files = self.get_project_data_file_paths()?;

        for project_data_file in project_data_files {
            let project =
                Project::parse_from_markdown_file(project_data_file).await?;

            self.cached_projects.push(project);
        }

        self.sort_cached_projects();

        Ok(())
    }

    pub fn get_cached_projects(&self) -> Vec<Project> {
        self.cached_projects.clone()
    }
}
