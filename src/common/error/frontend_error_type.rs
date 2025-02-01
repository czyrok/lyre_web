use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum FrontedErrorType {
    Unknown(String),
    UnknownProject(String),
    ProjectNotFound(String),
}

impl FrontedErrorType {
    pub fn unknown_project() -> Self {
        Self::UnknownProject("UNKNOWN_PROJECT".into())
    }

    pub fn project_not_found() -> Self {
        Self::ProjectNotFound("PROJECT_NOT_FOUND".into())
    }
}

impl Default for FrontedErrorType {
    fn default() -> Self {
        FrontedErrorType::Unknown("UNKNOWN_ERROR".into())
    }
}
