use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum FrontedErrorType {
    Unknown(String),
    UnknownProject(String),
    ProjectNotFound(String),
    RefreshProjectCacheFailed(String),
    UnableToGetProjectSlugs(String),
    UnableToGetProjectContexts(String),
    UnableToCheckIfProjectExists(String),
    UnableToGetProject(String),
    InvalidTotpToken(String),
}

impl FrontedErrorType {
    pub fn unknown_project() -> Self {
        Self::UnknownProject("UNKNOWN_PROJECT".into())
    }

    pub fn project_not_found() -> Self {
        Self::ProjectNotFound("PROJECT_NOT_FOUND".into())
    }

    pub fn refresh_project_cache_failed() -> Self {
        Self::RefreshProjectCacheFailed("REFRESH_PROJECT_CACHE_FAILED".into())
    }

    pub fn unable_to_get_project_slugs() -> Self {
        Self::UnableToGetProjectSlugs("UNABLE_TO_GET_PROJECT_SLUGS".into())
    }

    pub fn unable_to_get_project_contexts() -> Self {
        Self::UnableToGetProjectContexts(
            "UNABLE_TO_GET_PROJECT_CONTEXTS".into(),
        )
    }

    pub fn unable_to_check_if_project_exists() -> Self {
        Self::UnableToCheckIfProjectExists(
            "UNABLE_TO_CHECK_PROJECT_EXISTS".into(),
        )
    }

    pub fn unable_to_get_project() -> Self {
        Self::UnableToGetProject("UNABLE_TO_GET_PROJECT".into())
    }

    pub fn invalid_totp_token() -> Self {
        Self::InvalidTotpToken("INVALID_TOTP_TOKEN".into())
    }
}

impl Default for FrontedErrorType {
    fn default() -> Self {
        FrontedErrorType::Unknown("UNKNOWN_ERROR".into())
    }
}
