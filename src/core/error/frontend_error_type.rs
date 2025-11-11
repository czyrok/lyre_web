use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FrontedErrorType {
    #[default]
    Unknown,
    ContextNotFound,
    UnknownProject,
    ProjectNotFound,
    RefreshProjectCacheFailed,
    UnableToGetProjectSlugs,
    UnableToGetProjectContexts,
    UnableToGetProject,
    InvalidTotpToken,
    UnableToGetProjectTags,
}
