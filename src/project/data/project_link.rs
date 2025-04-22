use serde::{Deserialize, Serialize};

use crate::shared::components::icon::IconSet;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProjectLink {
    pub url: String,
    pub title: String,
    pub icon: Option<IconSet>,
}
