use serde::{Deserialize, Serialize};

use crate::core::data::icon_set::IconSet;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProjectLink {
    pub url: String,
    pub title: String,
    pub icon: Option<IconSet>,
}
